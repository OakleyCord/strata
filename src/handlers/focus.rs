use crate::{
	state::StrataState,
	workspaces::FocusTarget,
};
use smithay::desktop::Window;
pub use smithay::{
	backend::input::KeyState,
	desktop::{
		LayerSurface,
		PopupKind,
	},
	input::{
		keyboard::{
			KeyboardTarget,
			KeysymHandle,
			ModifiersState,
		},
		pointer::{
			AxisFrame,
			ButtonEvent,
			MotionEvent,
			PointerTarget,
			RelativeMotionEvent,
		},
		Seat,
	},
	reexports::wayland_server::{
		backend::ObjectId,
		protocol::wl_surface::WlSurface,
		Resource,
	},
	utils::{
		IsAlive,
		Serial,
	},
	wayland::seat::WaylandFocus,
};

impl IsAlive for FocusTarget {
	fn alive(&self) -> bool {
		match self {
			FocusTarget::Window(w) => w.alive(),
			FocusTarget::LayerSurface(l) => l.alive(),
			FocusTarget::Popup(p) => p.alive(),
		}
	}
}

impl From<FocusTarget> for WlSurface {
	fn from(target: FocusTarget) -> Self {
		target.wl_surface().unwrap()
	}
}

impl PointerTarget<StrataState> for FocusTarget {
	fn enter(&self, seat: &Seat<StrataState>, data: &mut StrataState, event: &MotionEvent) {
		match self {
			FocusTarget::Window(w) => PointerTarget::enter(w, seat, data, event),
			FocusTarget::LayerSurface(l) => PointerTarget::enter(l, seat, data, event),
			FocusTarget::Popup(p) => PointerTarget::enter(p.wl_surface(), seat, data, event),
		}
	}
	fn motion(&self, seat: &Seat<StrataState>, data: &mut StrataState, event: &MotionEvent) {
		match self {
			FocusTarget::Window(w) => PointerTarget::motion(w, seat, data, event),
			FocusTarget::LayerSurface(l) => PointerTarget::motion(l, seat, data, event),
			FocusTarget::Popup(p) => PointerTarget::motion(p.wl_surface(), seat, data, event),
		}
	}
	fn relative_motion(
		&self,
		seat: &Seat<StrataState>,
		data: &mut StrataState,
		event: &RelativeMotionEvent,
	) {
		match self {
			FocusTarget::Window(w) => PointerTarget::relative_motion(w, seat, data, event),
			FocusTarget::LayerSurface(l) => PointerTarget::relative_motion(l, seat, data, event),
			FocusTarget::Popup(p) => {
				PointerTarget::relative_motion(p.wl_surface(), seat, data, event)
			}
		}
	}
	fn button(&self, seat: &Seat<StrataState>, data: &mut StrataState, event: &ButtonEvent) {
		match self {
			FocusTarget::Window(w) => PointerTarget::button(w, seat, data, event),
			FocusTarget::LayerSurface(l) => PointerTarget::button(l, seat, data, event),
			FocusTarget::Popup(p) => PointerTarget::button(p.wl_surface(), seat, data, event),
		}
	}
	fn axis(&self, seat: &Seat<StrataState>, data: &mut StrataState, frame: AxisFrame) {
		match self {
			FocusTarget::Window(w) => PointerTarget::axis(w, seat, data, frame),
			FocusTarget::LayerSurface(l) => PointerTarget::axis(l, seat, data, frame),
			FocusTarget::Popup(p) => PointerTarget::axis(p.wl_surface(), seat, data, frame),
		}
	}
	fn leave(&self, seat: &Seat<StrataState>, data: &mut StrataState, serial: Serial, time: u32) {
		match self {
			FocusTarget::Window(w) => PointerTarget::leave(w, seat, data, serial, time),
			FocusTarget::LayerSurface(l) => PointerTarget::leave(l, seat, data, serial, time),
			FocusTarget::Popup(p) => PointerTarget::leave(p.wl_surface(), seat, data, serial, time),
		}
	}
	fn gesture_swipe_update(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GestureSwipeUpdateEvent,
	) {
		todo!()
	}
	fn gesture_swipe_end(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GestureSwipeEndEvent,
	) {
		todo!()
	}
	fn gesture_swipe_begin(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GestureSwipeBeginEvent,
	) {
		todo!()
	}
	fn gesture_pinch_update(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GesturePinchUpdateEvent,
	) {
		todo!()
	}
	fn gesture_pinch_end(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GesturePinchEndEvent,
	) {
		todo!()
	}
	fn gesture_pinch_begin(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GesturePinchBeginEvent,
	) {
		todo!()
	}
	fn gesture_hold_begin(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GestureHoldBeginEvent,
	) {
		todo!()
	}
	fn gesture_hold_end(
		&self,
		_: &smithay::input::Seat<StrataState>,
		_: &mut StrataState,
		_: &smithay::input::pointer::GestureHoldEndEvent,
	) {
		todo!()
	}

	fn frame(&self, seat: &Seat<StrataState>, data: &mut StrataState) {
		match self {
			FocusTarget::Window(w) => PointerTarget::frame(w, seat, data),
			FocusTarget::LayerSurface(l) => PointerTarget::frame(l, seat, data),
			FocusTarget::Popup(p) => PointerTarget::frame(p.wl_surface(), seat, data),
		}
	}
}

impl KeyboardTarget<StrataState> for FocusTarget {
	fn enter(
		&self,
		seat: &Seat<StrataState>,
		data: &mut StrataState,
		keys: Vec<KeysymHandle<'_>>,
		serial: Serial,
	) {
		match self {
			FocusTarget::Window(w) => KeyboardTarget::enter(w, seat, data, keys, serial),
			FocusTarget::LayerSurface(l) => KeyboardTarget::enter(l, seat, data, keys, serial),
			FocusTarget::Popup(p) => {
				KeyboardTarget::enter(p.wl_surface(), seat, data, keys, serial)
			}
		}
	}
	fn leave(&self, seat: &Seat<StrataState>, data: &mut StrataState, serial: Serial) {
		match self {
			FocusTarget::Window(w) => KeyboardTarget::leave(w, seat, data, serial),
			FocusTarget::LayerSurface(l) => KeyboardTarget::leave(l, seat, data, serial),
			FocusTarget::Popup(p) => KeyboardTarget::leave(p.wl_surface(), seat, data, serial),
		}
	}
	fn key(
		&self,
		seat: &Seat<StrataState>,
		data: &mut StrataState,
		key: KeysymHandle<'_>,
		state: KeyState,
		serial: Serial,
		time: u32,
	) {
		match self {
			FocusTarget::Window(w) => KeyboardTarget::key(w, seat, data, key, state, serial, time),
			FocusTarget::LayerSurface(l) => {
				KeyboardTarget::key(l, seat, data, key, state, serial, time)
			}
			FocusTarget::Popup(p) => {
				KeyboardTarget::key(p.wl_surface(), seat, data, key, state, serial, time)
			}
		}
	}
	fn modifiers(
		&self,
		seat: &Seat<StrataState>,
		data: &mut StrataState,
		modifiers: ModifiersState,
		serial: Serial,
	) {
		match self {
			FocusTarget::Window(w) => KeyboardTarget::modifiers(w, seat, data, modifiers, serial),
			FocusTarget::LayerSurface(l) => {
				KeyboardTarget::modifiers(l, seat, data, modifiers, serial)
			}
			FocusTarget::Popup(p) => {
				KeyboardTarget::modifiers(p.wl_surface(), seat, data, modifiers, serial)
			}
		}
	}
}

impl WaylandFocus for FocusTarget {
	fn wl_surface(&self) -> Option<WlSurface> {
		match self {
			FocusTarget::Window(w) => w.wl_surface(),
			FocusTarget::LayerSurface(l) => Some(l.wl_surface().clone()),
			FocusTarget::Popup(p) => Some(p.wl_surface().clone()),
		}
	}
	fn same_client_as(&self, object_id: &ObjectId) -> bool {
		match self {
			FocusTarget::Window(w) => w.same_client_as(object_id),
			FocusTarget::LayerSurface(l) => l.wl_surface().id().same_client_as(object_id),
			FocusTarget::Popup(p) => p.wl_surface().id().same_client_as(object_id),
		}
	}
}

impl From<Window> for FocusTarget {
	fn from(w: Window) -> Self {
		FocusTarget::Window(w)
	}
}

impl From<LayerSurface> for FocusTarget {
	fn from(l: LayerSurface) -> Self {
		FocusTarget::LayerSurface(l)
	}
}

impl From<PopupKind> for FocusTarget {
	fn from(p: PopupKind) -> Self {
		FocusTarget::Popup(p)
	}
}
