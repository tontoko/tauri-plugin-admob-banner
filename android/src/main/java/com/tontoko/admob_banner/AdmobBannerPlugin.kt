package com.tontoko.admob_banner

import android.app.Activity
import android.view.ViewGroup
import android.widget.FrameLayout
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.google.android.gms.ads.AdRequest
import com.google.android.gms.ads.AdSize
import com.google.android.gms.ads.AdView
import com.google.android.gms.ads.MobileAds
import com.google.android.ump.ConsentInformation
import com.google.android.ump.ConsentRequestParameters
import com.google.android.ump.UserMessagingPlatform

@InvokeArg
class ShowBannerArgs {
    lateinit var adUnitId: String
}

@TauriPlugin
class AdmobBannerPlugin(private val activity: Activity): Plugin(activity) {

    private var adView: AdView? = null
    private var consentInformation: ConsentInformation? = null

    @Command
    fun initialize(invoke: Invoke) {
        val params = ConsentRequestParameters.Builder()
            .setTagForUnderAgeOfConsent(false)
            .build()

        val consentInfo = UserMessagingPlatform.getConsentInformation(activity)
        consentInformation = consentInfo

        consentInfo.requestConsentInfoUpdate(
            activity,
            params,
            {
                if (consentInfo.isConsentFormAvailable) {
                    UserMessagingPlatform.loadAndShowConsentFormIfRequired(activity) { _ ->
                        invoke.resolve(JSObject().put("can_request_ads", consentInfo.canRequestAds()))
                    }
                } else {
                    invoke.resolve(JSObject().put("can_request_ads", consentInfo.canRequestAds()))
                }
            },
            { _ ->
                invoke.resolve(JSObject().put("can_request_ads", consentInfo.canRequestAds()))
            }
        )

        MobileAds.initialize(activity) { _ -> }
    }

    @Command
    fun show_banner(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(ShowBannerArgs::class.java)
            showBanner(args.adUnitId)
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Failed to show banner: ${e.message}")
        }
    }

    @Command
    fun hide_banner(invoke: Invoke) {
        hideBanner()
        invoke.resolve()
    }

    @Command
    fun can_request_ads(invoke: Invoke) {
        val can = consentInformation?.canRequestAds() ?: true
        invoke.resolve(JSObject().put("value", can))
    }

    @Command
    fun privacy_options_required(invoke: Invoke) {
        // UMP 4.0.0 changed the privacy options API significantly.
        // Always return false (not required) as a safe default; callers
        // can always call show_privacy_options if they want to offer it.
        invoke.resolve(JSObject().put("value", false))
    }

    @Command
    fun show_privacy_options(invoke: Invoke) {
        val info = consentInformation
        if (info == null) {
            invoke.resolve()
            return
        }
        UserMessagingPlatform.showPrivacyOptionsForm(activity) { _ ->
            invoke.resolve()
        }
    }

    @Synchronized
    private fun showBanner(adUnitId: String) {
        hideBanner()

        val view = AdView(activity)
        view.setAdSize(AdSize.BANNER)
        view.adUnitId = adUnitId

        val params = FrameLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.WRAP_CONTENT
        )
        params.gravity = android.view.Gravity.BOTTOM

        val rootView = activity.findViewById<FrameLayout>(android.R.id.content)
        rootView.addView(view, params)
        adView = view

        view.loadAd(AdRequest.Builder().build())
    }

    @Synchronized
    private fun hideBanner() {
        adView?.let { view ->
            (view.parent as? ViewGroup)?.removeView(view)
            view.destroy()
            adView = null
        }
    }
}
