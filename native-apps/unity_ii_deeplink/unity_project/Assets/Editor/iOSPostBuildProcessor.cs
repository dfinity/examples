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

        const string kURLScheme = "org.dfinity.unity-ii";
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
            itemDict.CreateArray(kURLSchemeKey).AddString(urlScheme);
            needsToWriteChanges = true;
        }
        else
        {
            // CFBundleURLTypes already exists (incremental build or another plugin).
            // Walk the entries and add our scheme if it is not already present.
            var urlTypesArray = rootDict.values[kURLTypesKey].AsArray();
            bool schemeFound = false;
            foreach (var item in urlTypesArray.values)
            {
                var dict = item.AsDict();
                if (dict == null || !dict.values.ContainsKey(kURLSchemeKey)) continue;
                foreach (var scheme in dict.values[kURLSchemeKey].AsArray().values)
                {
                    if (scheme.AsString() == urlScheme) { schemeFound = true; break; }
                }
                if (schemeFound) break;
            }
            if (!schemeFound)
            {
                var itemDict = urlTypesArray.AddDict();
                itemDict.SetString(kURLNameKey, urlIdentifier);
                itemDict.CreateArray(kURLSchemeKey).AddString(urlScheme);
                needsToWriteChanges = true;
            }
        }

        if (needsToWriteChanges)
        {
            File.WriteAllText(plistPath, plist.WriteToString());
        }
    }
}
#endif
