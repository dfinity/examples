using EdjCase.ICP.Agent.Agents.Http;
using System;
using System.Net;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.Networking;

public class UnityHttpClient : IHttpClient
{
    public async Task<HttpResponse> GetAsync(string url, CancellationToken? cancellationToken = null)
    {
        using (UnityWebRequest request = UnityWebRequest.Get(GetUri(url)))
        {
            await request.SendWebRequest();
            return ParseResponse(request);
        }
    }

    public async Task<HttpResponse> PostAsync(string url, byte[] cborBody, CancellationToken? cancellationToken = null)
    {
        using (UnityWebRequest request = new())
        {
            request.method = "POST";
            request.uri = GetUri(url);
            request.downloadHandler = new DownloadHandlerBuffer();

            request.uploadHandler = new UploadHandlerRaw(cborBody);
            request.uploadHandler.contentType = "application/cbor";
            await request.SendWebRequest();
            return ParseResponse(request);
        }
    }

    private static Uri GetUri(string path)
    {
        if (!path.StartsWith("/"))
        {
            path = "/" + path;
        }
        return new Uri("https://ic0.app" + path);
    }

    private static HttpResponse ParseResponse(UnityWebRequest request)
    {
        if (request.result != UnityWebRequest.Result.Success)
        {
            throw new Exception("Failed UnityWebRequest: " + request.error);
        }
        HttpStatusCode statusCode = (HttpStatusCode)request.responseCode;
        byte[] data = request.downloadHandler.data;
        return new HttpResponse(statusCode, () => Task.FromResult(data));
    }

}


internal class UnityWebRequestAwaiter : INotifyCompletion
{
    private UnityWebRequestAsyncOperation asyncOp;
    private Action continuation;

    public UnityWebRequestAwaiter(UnityWebRequestAsyncOperation asyncOp)
    {
        this.asyncOp = asyncOp;
        asyncOp.completed += OnRequestCompleted;
    }

    public bool IsCompleted { get { return asyncOp.isDone; } }

    public void GetResult() { }

    public void OnCompleted(Action continuation)
    {
        this.continuation = continuation;
    }

    private void OnRequestCompleted(AsyncOperation obj)
    {
        continuation();
    }
}
internal static class ExtensionMethods
{
    public static UnityWebRequestAwaiter GetAwaiter(this UnityWebRequestAsyncOperation asyncOp)
    {
        return new UnityWebRequestAwaiter(asyncOp);
    }
}


