using Microsoft.JSInterop;

namespace BlazorFrontend.Services;

/// <summary>
/// Calls the Motoko backend canister on ICP via JavaScript interop.
/// The IcpAgent global is defined in wwwroot/icpAgent.js (webpack bundle).
/// </summary>
public class IcpAgentService
{
    private readonly IJSRuntime _js;

    public IcpAgentService(IJSRuntime js)
    {
        _js = js;
    }

    public async Task<string> GetGreetingAsync()
        => await _js.InvokeAsync<string>("IcpAgent.getGreeting");

    public async Task<string> SetGreetingAsync(string name)
        => await _js.InvokeAsync<string>("IcpAgent.setGreeting", name);

    public async Task<string> HelloAsync(string name)
        => await _js.InvokeAsync<string>("IcpAgent.hello", name);
}
