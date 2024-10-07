//
//  WebViewNotificationApp.swift
//  WebViewNotification
//
//  Created by Kepler Vital on 08.12.22.
//

import SwiftUI

@main
struct DApp: App {
    @UIApplicationDelegateAdaptor var appDelegate: DAppDelegate

    var body: some Scene {
        WindowGroup {
            ContentView().environmentObject(DappConfig.shared)
        }
    }
}
