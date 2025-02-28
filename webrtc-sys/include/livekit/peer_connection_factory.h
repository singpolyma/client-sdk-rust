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

#include "api/peer_connection_interface.h"
#include "media_stream.h"
#include "peer_connection.h"
#include "rtp_parameters.h"
#include "rust/cxx.h"
#include "webrtc.h"

namespace livekit {
using NativeRTCConfiguration =
    webrtc::PeerConnectionInterface::RTCConfiguration;

class PeerConnectionFactory;
}  // namespace livekit
#include "webrtc-sys/src/peer_connection_factory.rs.h"

namespace livekit {

class PeerConnectionFactory {
 public:
  explicit PeerConnectionFactory(std::shared_ptr<RTCRuntime> rtc_runtime);
  ~PeerConnectionFactory();

  std::shared_ptr<PeerConnection> create_peer_connection(
      std::unique_ptr<NativeRTCConfiguration> config,
      NativePeerConnectionObserver* observer) const;

  std::shared_ptr<VideoTrack> create_video_track(
      rust::String label,
      std::shared_ptr<AdaptedVideoTrackSource> source) const;

  std::shared_ptr<AudioTrack> create_audio_track(
      rust::String label,
      std::shared_ptr<AudioTrackSource> source) const;

  RtpCapabilities get_rtp_sender_capabilities(MediaType type) const;

  RtpCapabilities get_rtp_receiver_capabilities(MediaType type) const;

 private:
  std::shared_ptr<RTCRuntime> rtc_runtime_;
  rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> peer_factory_;
};

std::shared_ptr<PeerConnectionFactory> create_peer_connection_factory(
    std::shared_ptr<RTCRuntime> rtc_runtime);
std::unique_ptr<NativeRTCConfiguration> create_rtc_configuration(
    RTCConfiguration conf);
}  // namespace livekit
