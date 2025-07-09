//
//  WebView.swift
//  WebViewNotification
//
//  Created by Kepler Vital on 08.12.22.
//

import SwiftUI
import Combine
import WebKit
import AuthenticationServices

/// A container for using a WKWebView in SwiftUI
public struct DAppWebView: UIViewRepresentable {
    /// The dapp to display
    public let dapp: DappConfig
    public let webView: WKWebView
    public let loading: LoadingScreen
    public var loginSession: LoginSession?
  
    public init(dapp: DappConfig) {
        self.dapp = dapp

        let configuration = WKWebViewConfiguration()
        configuration.limitsNavigationsToAppBoundDomains = true
        configuration.preferences.javaScriptCanOpenWindowsAutomatically = true
        self.webView = WKWebView(frame: .zero, configuration: configuration)
        self.loading = LoadingScreen.create()
    }
  
    public func makeUIView(context: UIViewRepresentableContext<DAppWebView>) -> WKWebView {
        webView.uiDelegate = context.coordinator
        webView.navigationDelegate = context.coordinator

        return webView
    }
  
    public func updateUIView(_ uiView: WKWebView, context: UIViewRepresentableContext<DAppWebView>) {
        uiView.window?.rootViewController?.present(self.loading, animated: true)

        let request = URLRequest(url: self.dapp.url)
        uiView.load(request)
    }
    
    public func makeCoordinator() -> WebViewCoordinator {
        return WebViewCoordinator(self)
    }
    
    public class WebViewCoordinator: NSObject, WKUIDelegate, WKNavigationDelegate {
        var parent: DAppWebView
        
        init(_ parent: DAppWebView) {
            self.parent = parent

            super.init()
        }
        
        public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
            parent.webView.window?.rootViewController?.dismiss(animated: false)
        }

        public func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
            parent.webView.window?.rootViewController?.dismiss(animated: false)
        }
        
        public func makeWKWebView(parentView: WKWebView, configuration: WKWebViewConfiguration, request: URLRequest) -> WKWebView {
            let view = WKWebView(frame: parentView.frame, configuration: configuration)
            parentView.addSubview(view)
            
            DispatchQueue.main.async {
                view.load(request)
            }
            return view
        }
        
        public func webView(_ webView: WKWebView, createWebViewWith configuration: WKWebViewConfiguration, for navigationAction: WKNavigationAction, windowFeatures: WKWindowFeatures) -> WKWebView? {
            if (navigationAction.targetFrame == nil) {
                parent.loginSession = LoginSession(self.parent)
                parent.loginSession?.start()

                return nil
            }
            
            return makeWKWebView(parentView: webView, configuration: configuration, request: navigationAction.request)
        }
    }
}
