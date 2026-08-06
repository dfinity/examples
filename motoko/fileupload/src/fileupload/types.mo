module {

    public type HeaderField = (Text, Text);

    public type Chunk = {
        batch_name : Text;
        content  : Blob;
    };

    public type Asset = {
        encoding: AssetEncoding;
        content_type: Text;
    };

    public type AssetEncoding = {
        modified       : Int;
        content_chunks : [Blob];
        total_length   : Nat;
        certified      : Bool;
    };

    public type HttpRequest = {
        url : Text;
        method : Text;
        body : Blob;
        headers : [HeaderField];
    };

    public type HttpResponse = {
        body : [Nat8];
        headers : [HeaderField];
        status_code : Nat16;
        streaming_strategy : ?StreamingStrategy;
    };

    public type StreamingStrategy = {
        #Callback : {
            token : StreamingCallbackToken;
            callback : shared () -> async ();
        };
    };

    public type StreamingCallbackToken = {
        key : Text;
        index : Nat;
        content_encoding : Text;
    };

    public type StreamingCallbackHttpResponse = {
        body : Blob;
        token: ?StreamingCallbackToken;
    };
}
