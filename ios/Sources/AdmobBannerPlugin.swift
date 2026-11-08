import Foundation
import GoogleMobileAds
import GoogleUserMessagingPlatform
import Tauri
import UIKit
import WebKit

/// `show_banner` arguments (contract: `{ adUnitId }`; the Rust side
/// serializes the field camelCase to match the native arg classes).
struct ShowBannerArgs: Decodable {
  let adUnitId: String
}

/// iOS implementation of the AdMob banner plugin. Mirrors the Android
/// (Kotlin) `AdmobBannerPlugin` command for command:
///
/// - `initialize` runs the UMP consent flow
///   (`requestConsentInfoUpdate` + `loadAndShowIfRequired`) and starts the
///   Google Mobile Ads SDK in parallel, resolving
///   `{ can_request_ads: Bool }` once the consent step settles. Like the
///   Kotlin side, consent errors do not reject — the invoke resolves with
///   whatever `canRequestAds` reports at that point.
/// - `show_banner` replaces any existing banner with an anchored adaptive
///   banner pinned to the bottom of the root view, above the safe area.
/// - `hide_banner` removes and releases the banner.
/// - `can_request_ads` / `privacy_options_required` resolve `{ value: Bool }`.
/// - `show_privacy_options` presents the UMP privacy options form.
///
/// The Tauri plugin manager dispatches commands on a background queue, so
/// every command hops to the main queue before touching UIKit, the Google
/// Mobile Ads SDK or plugin state; all stored properties are therefore
/// main-queue-confined.
class AdmobBannerPlugin: Plugin {
  /// The live banner view, if any. A new `show_banner` replaces it.
  private var bannerView: GADBannerView?
  /// Ad unit of the current banner, kept so an orientation change can
  /// rebuild the banner (the adaptive size depends on the width).
  private var currentAdUnitId: String?

  /// Weak on purpose: the WebView is owned by the Tauri runtime.
  private weak var webView: WKWebView?

  /// Called by the Tauri plugin manager when the WebView is created;
  /// captures the WebView as a fallback anchor for the banner container.
  public override func load(webview: WKWebView) {
    self.webView = webview
    NotificationCenter.default.addObserver(
      self,
      selector: #selector(orientationDidChange),
      name: UIDevice.orientationDidChangeNotification,
      object: nil
    )
  }

  // MARK: - Commands

  @objc public func initialize(_ invoke: Invoke) throws {
    DispatchQueue.main.async { [weak self] in
      guard let self else {
        invoke.reject("plugin deallocated")
        return
      }

      let parameters = UMPRequestParameters()
      parameters.tagForUnderAgeOfConsent = false

      let consentInformation = UMPConsentInformation.sharedInstance
      consentInformation.requestConsentInfoUpdate(with: parameters) { _ in
        // UMP delivers its callbacks on the main queue; hop anyway so all
        // state access stays main-queue-confined. Both the success and the
        // error path resolve with the current `canRequestAds`, exactly
        // like the Kotlin implementation.
        DispatchQueue.main.async {
          if consentInformation.formStatus == .available,
            let viewController = self.presentingViewController()
          {
            UMPConsentForm.loadAndShowIfRequired(from: viewController) { _ in
              invoke.resolve(["can_request_ads": consentInformation.canRequestAds])
            }
          } else {
            invoke.resolve(["can_request_ads": consentInformation.canRequestAds])
          }
        }
      }

      // Mirrors the Kotlin side: the GMA SDK start is kicked off
      // immediately, in parallel with the consent flow, and its
      // completion is not awaited before resolving.
      GADMobileAds.sharedInstance().start { _ in }
    }
  }

  @objc public func show_banner(_ invoke: Invoke) throws {
    let args: ShowBannerArgs
    do {
      args = try invoke.parseArgs(ShowBannerArgs.self)
    } catch {
      invoke.reject("Failed to show banner: \(error.localizedDescription)")
      return
    }

    DispatchQueue.main.async { [weak self] in
      guard let self else {
        invoke.reject("plugin deallocated")
        return
      }
      guard let container = self.containerView() else {
        invoke.reject("Failed to show banner: no view available to attach the banner")
        return
      }
      self.showBanner(adUnitId: args.adUnitId, in: container)
      invoke.resolve()
    }
  }

  @objc public func hide_banner(_ invoke: Invoke) throws {
    DispatchQueue.main.async { [weak self] in
      self?.hideBanner()
      invoke.resolve()
    }
  }

  @objc public func can_request_ads(_ invoke: Invoke) throws {
    invoke.resolve(["value": UMPConsentInformation.sharedInstance.canRequestAds])
  }

  @objc public func privacy_options_required(_ invoke: Invoke) throws {
    invoke.resolve([
      "value": UMPConsentInformation.sharedInstance.privacyOptionsRequirementStatus
        == .required
    ])
  }

  @objc public func show_privacy_options(_ invoke: Invoke) throws {
    DispatchQueue.main.async { [weak self] in
      // Mirrors the Kotlin side: with no view controller to present from
      // there is nothing to show, and the invoke simply resolves.
      guard let self, let viewController = self.presentingViewController() else {
        invoke.resolve()
        return
      }
      UMPConsentForm.presentPrivacyOptionsForm(from: viewController) { _ in
        invoke.resolve()
      }
    }
  }

  // MARK: - Banner lifecycle (main queue only)

  private func showBanner(adUnitId: String, in container: UIView) {
    hideBanner()
    currentAdUnitId = adUnitId

    // Anchored adaptive size for the current orientation, sized from the
    // container width (the screen width while layout has not run yet).
    let width =
      container.bounds.width > 0 ? container.bounds.width : UIScreen.main.bounds.width
    let adSize = GADCurrentOrientationAnchoredAdaptiveBannerAdSizeWithWidth(width)

    let banner = GADBannerView(adSize: adSize)
    banner.adUnitID = adUnitId
    banner.rootViewController = presentingViewController()
    banner.translatesAutoresizingMaskIntoConstraints = false
    container.addSubview(banner)

    // Keep the banner above the home indicator / gesture area by pinning
    // it to the safe area, the iOS counterpart of the window-inset margins
    // the Kotlin side applies for the navigation bar and display cutout.
    NSLayoutConstraint.activate([
      banner.leadingAnchor.constraint(equalTo: container.leadingAnchor),
      banner.trailingAnchor.constraint(equalTo: container.trailingAnchor),
      banner.bottomAnchor.constraint(equalTo: container.safeAreaLayoutGuide.bottomAnchor),
      banner.heightAnchor.constraint(equalToConstant: adSize.size.height),
    ])
    bannerView = banner

    banner.load(GADRequest())
  }

  private func hideBanner() {
    bannerView?.removeFromSuperview()
    bannerView = nil
    currentAdUnitId = nil
  }

  /// The anchored adaptive size depends on the current width, so a
  /// rotation rebuilds the banner with the same ad unit once the new
  /// layout has settled.
  @objc private func orientationDidChange() {
    DispatchQueue.main.async { [weak self] in
      guard let self,
        let banner = self.bannerView,
        let adUnitId = self.currentAdUnitId,
        let container = banner.superview
      else {
        return
      }
      let width =
        container.bounds.width > 0 ? container.bounds.width : UIScreen.main.bounds.width
      if abs(banner.adSize.size.width - width) > 1 {
        self.showBanner(adUnitId: adUnitId, in: container)
      }
    }
  }

  // MARK: - View lookup

  /// View controller used to present UMP forms: the Tauri plugin
  /// manager's view controller, falling back to the root of the window
  /// hosting the WebView.
  private func presentingViewController() -> UIViewController? {
    if let viewController = manager.viewController {
      return viewController
    }
    return webView?.window?.rootViewController
  }

  /// View the banner is attached to: the Tauri root view, falling back
  /// to the WebView's superview (captured in `load(webview:)`).
  private func containerView() -> UIView? {
    if let view = manager.viewController?.view {
      return view
    }
    return webView?.superview
  }
}

/// Exported C symbol registered by the Rust core through
/// `register_ios_plugin(init_plugin_admob_banner)`
/// (`tauri::ios_plugin_binding!`).
@_cdecl("init_plugin_admob_banner")
func initPlugin() -> Plugin {
  return AdmobBannerPlugin()
}
