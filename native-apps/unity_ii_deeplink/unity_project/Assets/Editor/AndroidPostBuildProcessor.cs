#if UNITY_ANDROID
using System;
using System.IO;
using System.Xml;
using UnityEditor.Android;

namespace IC.GameKit
{
    public class AndroidPostProcessor : IPostGenerateGradleAndroidProject
    {
        const string kAndroidNamespaceURI = "http://schemas.android.com/apk/res/android";

        // Android URL Scheme, you can add more parameter like port etc..
        const string kAndroidScheme = "internetidentity";
        const string kAndroidHost = "authorize";

        public int callbackOrder { get { return 0; } }

        public void OnPostGenerateGradleAndroidProject(string projectPath)
        {
            InjectAndroidManifest(projectPath);
        }

        private void InjectAndroidManifest(string projectPath)
        {
            var manifestPath = projectPath + "/src/main/AndroidManifest.xml";
            if (!File.Exists(manifestPath))
                throw new FileNotFoundException(manifestPath + " doesn't exist.");

            var manifestXmlDoc = new XmlDocument();
            manifestXmlDoc.Load(manifestPath);

            AppendAndroidIntentFilter(manifestPath, manifestXmlDoc);

            manifestXmlDoc.Save(manifestPath);
        }

        internal static void AppendAndroidIntentFilter(string manifestPath, XmlDocument xmlDoc)
        {
            var activityNode = xmlDoc.SelectSingleNode("manifest/application/activity");
            if (activityNode == null)
                throw new ArgumentException(string.Format("Missing 'activity' node in '{0}'.", manifestPath));

            // TODO: we need to check if the same url scheme has already existed before injecting.
            var intentFilterNode = xmlDoc.CreateElement("intent-filter");
            
            var actionNode = xmlDoc.CreateElement("action");
            actionNode.SetAttribute("name", kAndroidNamespaceURI, "android.intent.action.VIEW");
            intentFilterNode.AppendChild(actionNode);

            var categoryNode1 = xmlDoc.CreateElement("category");
            categoryNode1.SetAttribute("name", kAndroidNamespaceURI, "android.intent.category.DEFAULT");
            intentFilterNode.AppendChild(categoryNode1);

            var categoryNode2 = xmlDoc.CreateElement("category");
            categoryNode2.SetAttribute("name", kAndroidNamespaceURI, "android.intent.category.BROWSABLE");
            intentFilterNode.AppendChild(categoryNode2);

            var dataNode = xmlDoc.CreateElement("data");
            dataNode.SetAttribute("host", kAndroidNamespaceURI, kAndroidHost);
            dataNode.SetAttribute("scheme", kAndroidNamespaceURI, kAndroidScheme);
            intentFilterNode.AppendChild(dataNode);

            activityNode.AppendChild(intentFilterNode);
        }
    }
}
#endif
