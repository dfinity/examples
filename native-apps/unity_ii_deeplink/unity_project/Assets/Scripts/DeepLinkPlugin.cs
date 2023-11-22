using UnityEngine;
using EdjCase.ICP.Candid.Utilities;
using Newtonsoft.Json;
using System.Web;

namespace IC.GameKit
{
    public class DeepLinkPlugin : MonoBehaviour
    {
        TestICPAgent mTestICPAgent = null;

        private void Awake()
        {
            // Register action for deep link activated.
            Application.deepLinkActivated += OnDeepLinkActivated;
        }

        public void Start()
        {
            mTestICPAgent = gameObject.GetComponent<TestICPAgent>();
        }

        public void OpenBrowser()
        {
            var target = mTestICPAgent.greetFrontend + "?sessionkey=" + ByteUtil.ToHexString(mTestICPAgent.TestIdentity.PublicKey.ToDerEncoding());
            Application.OpenURL(target);
        }

        public void OnDeepLinkActivated(string url)
        {
            if (string.IsNullOrEmpty(url))
                return;

            const string kDelegationParam = "delegation=";
            var indexOfDelegation = url.IndexOf(kDelegationParam);
            if (indexOfDelegation == -1)
            {
                Debug.LogError("Cannot find delegation");
                return;
            }

            var delegationString = HttpUtility.UrlDecode(url.Substring(indexOfDelegation + kDelegationParam.Length));
            var delegation = JsonConvert.DeserializeObject<DelegationChainModel>(delegationString);
            mTestICPAgent.Delegation = delegation;
        }
    }
}
