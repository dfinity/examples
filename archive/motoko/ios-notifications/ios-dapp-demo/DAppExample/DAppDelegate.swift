//
//  AppDelegate.swift
//  DAppExample
//
//  Created by Kepler Vital on 15.12.22.
//

import SwiftUI

extension Data {
    var hexString: String {
        let hexString = map { String(format: "%02.2hhx", $0) }.joined()
        return hexString
    }
}

class DAppDelegate: NSObject, UIApplicationDelegate, UNUserNotificationCenterDelegate {
    private let dapp = DappConfig.shared
    
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil) -> Bool {
        let center = UNUserNotificationCenter.current()
        center.delegate = self
        center.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
            if let error = error {
                // Handle the error here.
                print(error)
            }

            DispatchQueue.main.async {
                UIApplication.shared.registerForRemoteNotifications()
            }
        }
        
        return true
    }
    
    func userNotificationCenter(_ center: UNUserNotificationCenter, willPresent notification: UNNotification, withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void) {
        completionHandler([.banner, .list, .badge])
    }
    
    func userNotificationCenter(_ center: UNUserNotificationCenter, didReceive response: UNNotificationResponse, withCompletionHandler completionHandler: @escaping () -> Void) {
        let aps = response.notification.request.content.userInfo["aps"] as? [String: AnyObject]
        let rawURL = aps?["alert"]?["url"]
        
        guard rawURL != nil, let deepLink = URL(string: rawURL as? String ?? "") else { return }
        
        if dapp.isAllowedDeepLink(url: deepLink) {
            dapp.url = deepLink
        }
        
        completionHandler()
    }

    func application(_ application: UIApplication, didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data) {
        dapp.deviceToken = deviceToken.hexString
        // for this example we print the device token to the console to be used in send-notification.sh, however, in a real
        // use case the token should be stored in a canister associated with the authenticated user
        print("device id: " + deviceToken.hexString)
    }
}
