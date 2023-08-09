package com.icgamekit.plugin;

import android.content.Intent;
import android.net.Uri;
import android.util.Log;

import com.unity3d.player.UnityPlayer;

import java.io.File;
import java.io.FileOutputStream;

public class ICGameKitPlugin {
    static final String TAG_PLUGIN = "ICGameKitPlugin";

    public static ICGameKitPlugin sCurrentPlugin;

    public static ICGameKitPlugin initImpl() {
        if (sCurrentPlugin != null)
            return sCurrentPlugin;

        sCurrentPlugin = new ICGameKitPlugin();

        return sCurrentPlugin;
    }

    public void openBrowser(String url) {
        Log.i(TAG_PLUGIN, url);
        
        //String url = "https://6x7nu-oaaaa-aaaan-qdaua-cai.ic0.app";
        Uri uri = Uri.parse(url);
        Intent intent = new Intent(Intent.ACTION_VIEW, uri);
        UnityPlayer.currentActivity.startActivity(intent);
    }

    public void sendMessage(String[] arguments) {
        if (arguments == null || arguments.length != 2)
            return;

        String gameObjectName = arguments[0];
        String methodName = arguments[1];

        if (gameObjectName == null || gameObjectName.length() == 0
            || methodName == null || methodName.length() == 0)
            return;

        Uri uri = UnityPlayer.currentActivity.getIntent().getData();
        if (uri == null)
            return;

        String url = uri.toString();
        int index = url.indexOf("delegation=");
        if (index == -1)
            return;

        String delegation = url.substring(index);
        //Log.i(TAG_PLUGIN, delegation);

        // Write to a temporary file to internal storage and read it back from C# side.
        // The reason is we can only pass 1024 bytes as string back to the C# side, but the params string with delegation is more than 3k bytes.
        String delegationPath = UnityPlayer.currentActivity.getFilesDir().getPath() + "/delegation.file";
        File delegationFile = new File(delegationPath);
        try {
            if (delegationFile.exists())
                delegationFile.delete();

            FileOutputStream fileOutputStream = new FileOutputStream(delegationFile);
            fileOutputStream.write(delegation.getBytes());
            fileOutputStream.flush();
            fileOutputStream.close();
        } catch (Exception e) {
            e.printStackTrace();
        }

        // Pass the delegation path back to C#.
        UnityPlayer.UnitySendMessage(gameObjectName, methodName, delegationPath);
    }
}
