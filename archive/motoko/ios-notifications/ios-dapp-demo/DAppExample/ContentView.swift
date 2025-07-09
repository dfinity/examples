//
//  ContentView.swift
//  WebViewNotification
//
//  Created by Kepler Vital on 08.12.22.
//

import SwiftUI
import WebKit

struct ContentView: View {
    @EnvironmentObject private var dapp: DappConfig
    
    var body: some View {
        DAppWebView(dapp: dapp)
            .onOpenURL{ url in
                guard url.path().hasPrefix(URL(string: dapp.authCallbackURL)!.path()) else { return }
                
                dapp.loginSession?.identityCallbackHook(successURL: url)
            }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
