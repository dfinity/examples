using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using BlazorFrontend.Services;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<BlazorFrontend.App>("#app");

builder.Services.AddScoped<IcpAgentService>();

await builder.Build().RunAsync();
