#if UNITY_IOS
using System.IO;
using UnityEditor;
using UnityEditor.Callbacks;
using UnityEditor.iOS.Xcode;
using UnityEngine;

public class iOSPostBuildProcessor : MonoBehaviour
{
    [PostProcessBuild]
    public static void OnPostprocessBuild(BuildTarget buildTarget, string path)
    {
        if (buildTarget != BuildTarget.iOS)
            return;

        const string kURLScheme = "internetidentity";
        const string kURLIdentifier = "authorize";

        // Patch the plist file to add URL schemes.
        PatchPlist(path, kURLScheme, kURLIdentifier);
    }

    private static void PatchPlist(string path, string urlScheme, string urlIdentifier)
    {
        var plistPath = path + "/Info.plist";
        var plist = new PlistDocument();
        plist.ReadFromString(File.ReadAllText(plistPath));

        var rootDict = plist.root;
        var needsToWriteChanges = false;

        const string kURLTypesKey = "CFBundleURLTypes";
        const string kURLNameKey = "CFBundleURLName";
        const string kURLSchemeKey = "CFBundleURLSchemes";

        if (!rootDict.values.ContainsKey(kURLTypesKey))
        {
            var urlTypesArray = rootDict.CreateArray(kURLTypesKey);
            var itemDict = urlTypesArray.AddDict();
            itemDict.SetString(kURLNameKey, urlIdentifier);
            var schemeArray = itemDict.CreateArray(kURLSchemeKey);
            schemeArray.AddString(urlScheme);

            needsToWriteChanges = true;
        }
        else
        {
            // TODO: Check if the url shceme has been updated.
        }

        if (needsToWriteChanges)
        {
            File.WriteAllText(plistPath, plist.WriteToString());
        }
    }
}
#endif
