use sync_wrapper::{SyncFuture, SyncWrapper};
use transform_stream::AsyncStream;

type AwsSelectObjectContentEventStream = aws_smithy_http::event_stream::Receiver<
    aws_sdk_s3::model::SelectObjectContentEventStream,
    aws_sdk_s3::error::SelectObjectContentEventStreamError,
>;

pub fn from_aws(src: AwsSelectObjectContentEventStream) -> s3s::dto::SelectObjectContentEventStream {
    let mut src = SyncWrapper::new(src);
    s3s::dto::SelectObjectContentEventStream::new(AsyncStream::new(|mut y| async move {
        loop {
            let recv = SyncFuture::new(src.get_mut().recv());
            match recv.await {
                Ok(Some(ev)) => y.yield_(crate::conv::try_from_aws(ev)).await,
                Ok(None) => break,
                Err(err) => y.yield_err(wrap_sdk_error!(err)).await,
            }
        }
    }))
}
