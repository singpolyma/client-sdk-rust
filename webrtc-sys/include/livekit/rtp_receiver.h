/*
 * Copyright 2023 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the “License”);
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an “AS IS” BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>

#include "api/rtp_receiver_interface.h"
#include "livekit/helper.h"
#include "livekit/media_stream.h"
#include "livekit/rtp_parameters.h"
#include "rust/cxx.h"

namespace livekit {
class RtpReceiver;
}
#include "webrtc-sys/src/rtp_receiver.rs.h"
namespace livekit {

// TODO(theomonnom): Implement RtpReceiverObserverInterface?
// TODO(theomonnom): RtpSource
// TODO(theomonnom): FrameTransformer & FrameDecryptor interface
class RtpReceiver {
 public:
  explicit RtpReceiver(
      rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver);

  std::shared_ptr<MediaStreamTrack> track() const;

  rust::Vec<rust::String> stream_ids() const;
  rust::Vec<MediaStreamPtr> streams() const;

  MediaType media_type() const;
  rust::String id() const;

  RtpParameters get_parameters() const;

  // bool set_parameters(RtpParameters parameters) const; // Seems unsupported

  void set_jitter_buffer_minimum_delay(bool is_some,
                                       double delay_seconds) const;

 private:
  rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver_;
};

static std::shared_ptr<RtpReceiver> _shared_rtp_receiver() {
  return nullptr;
}

}  // namespace livekit
