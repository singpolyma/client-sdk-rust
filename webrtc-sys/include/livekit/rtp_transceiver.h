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

#include "api/rtp_parameters.h"
#include "api/rtp_transceiver_direction.h"
#include "api/rtp_transceiver_interface.h"
#include "livekit/rtc_error.h"
#include "livekit/rtp_parameters.h"
#include "livekit/rtp_receiver.h"
#include "livekit/rtp_sender.h"
#include "rust/cxx.h"

namespace livekit {
class RtpTransceiver;
}
#include "webrtc-sys/src/rtp_transceiver.rs.h"

namespace livekit {

webrtc::RtpTransceiverInit to_native_rtp_transceiver_init(
    RtpTransceiverInit init);

class RtpTransceiver {
 public:
  explicit RtpTransceiver(
      rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver);

  MediaType media_type() const;

  rust::String mid() const;

  std::shared_ptr<RtpSender> sender() const;

  std::shared_ptr<RtpReceiver> receiver() const;

  bool stopped() const;

  bool stopping() const;

  RtpTransceiverDirection direction() const;

  void set_direction(RtpTransceiverDirection direction) const;

  RtpTransceiverDirection current_direction() const;

  RtpTransceiverDirection fired_direction() const;

  void stop_standard() const;

  void set_codec_preferences(rust::Vec<RtpCodecCapability> codecs) const;

  rust::Vec<RtpCodecCapability> codec_preferences() const;

  rust::Vec<RtpHeaderExtensionCapability> header_extensions_to_offer() const;

  rust::Vec<RtpHeaderExtensionCapability> header_extensions_negotiated() const;

  void set_offered_rtp_header_extensions(
      rust::Vec<RtpHeaderExtensionCapability> header_extensions_to_offer) const;

 private:
  rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver_;
};

static std::shared_ptr<RtpTransceiver> _shared_rtp_transceiver() {
  return nullptr;
}

}  // namespace livekit
