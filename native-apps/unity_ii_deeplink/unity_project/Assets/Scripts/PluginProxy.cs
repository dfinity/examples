using UnityEngine;
using EdjCase.ICP.Candid.Utilities;

namespace IC.GameKit
{
    public class PluginProxy : MonoBehaviour
    {
        const string kGameObjectName = "AgentAndPlugin";
        const string kMethodName = "OnMessageSent";

        public string greetFrontend = "https://6x7nu-oaaaa-aaaan-qdaua-cai.icp0.io/";        

        TestICPAgent mTestICPAgent = null;

#if UNITY_ANDROID
        private AndroidJavaObject mPlugin = null;
#endif

        public void Start()
        {
            mTestICPAgent = gameObject.GetComponent<TestICPAgent>();

#if UNITY_ANDROID
            var pluginClass = new AndroidJavaClass("com.icgamekit.plugin.ICGameKitPlugin");
            mPlugin = pluginClass.CallStatic<AndroidJavaObject>("initImpl");
#endif
        }

        public void OpenBrowser()
        {
            var target = greetFrontend + "?sessionkey=" + ByteUtil.ToHexString(mTestICPAgent.TestIdentity.PublicKey.Value);

#if UNITY_ANDROID
            mPlugin.Call("openBrowser", target);
#endif
        }

        public void OnApplicationPause(bool pause)
        {
            // If it's resuming.
            if (!pause)
            {
#if UNITY_ANDROID
                // OnApplicationPause will be called while launching the app, before mPlugin is initialized.
                if (mPlugin == null)
                    return;

                mPlugin.Call("sendMessage", new string[] { kGameObjectName, kMethodName });
#endif
            }
        }
    }
}
