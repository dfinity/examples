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

        PatchPBXProject(path);
    }

    private static void PatchPBXProject(string projectPath)
    {
        var pbxProjectPath = PBXProject.GetPBXProjectPath(projectPath);
        var needUpdatePbxProjectFile = false;

        var pbxProject = new PBXProject();
        pbxProject.ReadFromString(File.ReadAllText(pbxProjectPath));

        // Get main target.
        string mainTarget;
        var unityMainTargetGuidMethod = pbxProject.GetType().GetMethod("GetUnityMainTargetGuid");
        if (unityMainTargetGuidMethod != null)
        {
            mainTarget = (string)unityMainTargetGuidMethod.Invoke(pbxProject, null);
        }
        else
        {
            mainTarget = pbxProject.TargetGuidByName("Unity-iPhone");
        }

        // Check if the pbxproj file already contains the entitlements file configuration.
        var entitlementsFileName = pbxProject.GetBuildPropertyForAnyConfig(mainTarget, "CODE_SIGN_ENTITLEMENTS");
        if (entitlementsFileName == null)
        {
            needUpdatePbxProjectFile = true;

            var bundleIdentifier = PlayerSettings.GetApplicationIdentifier(BuildTargetGroup.iOS);
            entitlementsFileName = string.Format("{0}.entitlements", bundleIdentifier.Substring(bundleIdentifier.LastIndexOf(".") + 1));
        }

        PatchEntitlementsFile(projectPath, entitlementsFileName);

        if (needUpdatePbxProjectFile)
        {
            // Update the new entitlements file info into the pbxproj file.
            pbxProject.AddBuildProperty(mainTarget, "CODE_SIGN_ENTITLEMENTS", entitlementsFileName);
            pbxProject.WriteToFile(pbxProjectPath);
        }
    }

    private static void PatchEntitlementsFile(string projectPath, string entitlementsFileName)
    {
        const string kAssociatedDomainsKey = "com.apple.developer.associated-domains";
        const string kURLScheme = "applinks";
        const string kURLIdentifier = "ms43p-uaaaa-aaaan-qixeq-cai.icp0.io";

        var entitlementsFile = new PlistDocument();
        var entitlementsFilePath = Path.Combine(projectPath, entitlementsFileName);
        if (File.Exists(entitlementsFilePath))
        {
            entitlementsFile.ReadFromFile(entitlementsFilePath);
        }

        var needUpdateEntitlementsFile = false;

        // Write the associated domains info into the entitlements file.
        var rootDict = entitlementsFile.root;
        if (!rootDict.values.ContainsKey(kAssociatedDomainsKey))
        {
            var domainsArray = rootDict.CreateArray(kAssociatedDomainsKey);
            domainsArray.AddString(string.Format("{0}:{1}", kURLScheme, kURLIdentifier));
            needUpdateEntitlementsFile = true;
        }
        else
        {
            // TODO: Check if the associated domains scheme has been updated.
        }

        if (needUpdateEntitlementsFile)
        {
            entitlementsFile.WriteToFile(entitlementsFilePath);
        }
    }
}
#endif
