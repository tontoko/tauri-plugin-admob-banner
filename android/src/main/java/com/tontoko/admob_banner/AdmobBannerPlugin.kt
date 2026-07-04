package com.tontoko.admob_banner

import android.app.Activity
import android.view.ViewGroup
import android.widget.FrameLayout
import com.google.android.gms.ads.AdRequest
import com.google.android.gms.ads.AdSize
import com.google.android.gms.ads.AdView
import com.google.android.gms.ads.MobileAds
import com.google.android.ump.ConsentDebugSettings
import com.google.android.ump.ConsentInformation
import com.google.android.ump.ConsentRequestParameters
import com.google.android.ump.UserMessagingPlatform

import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class AdmobBannerPlugin(private val activity: Activity): Plugin(activity) {

    private var adView: AdView? = null
    private var consentInformation: ConsentInformation? = null

    @Command
    fun initialize(invoke: Invoke) {
        val canRequest = consentInformation?.canRequestAds() ?: true
        invoke.resolve(JSObject().put("can_request_ads", canRequest))
    }

    fun runInitialize(testDeviceIds: List<String>, callback: (Boolean) -> Unit) {
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
                        callback(consentInfo.canRequestAds())
                    }
                } else {
                    callback(consentInfo.canRequestAds())
                }
            },
            { _ ->
                callback(consentInfo.canRequestAds())
            }
        )

        MobileAds.initialize(activity) { _ -> }
    }

    @Command
    fun show_banner(invoke: Invoke) {
        val adUnitId = invoke.getString("ad_unit_id") ?: ""
        showBanner(adUnitId)
        invoke.resolve()
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
        val info = consentInformation
        val required = info != null && info.privacyOptionsRequirementFlags
            .get(ConsentInformation.PrivacyOptionsRequirementFlag.REQUIRED)
        invoke.resolve(JSObject().put("value", required))
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
