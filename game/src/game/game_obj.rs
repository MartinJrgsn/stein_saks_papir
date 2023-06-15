pub trait GameObj<SessionType, UIType>
where
    SessionType: SessionObj + ?Sized,
    UIType: UI + ?Sized
{
    fn get_session(self: &Self) -> &SessionType;
    fn get_session_mut(self: &mut Self) -> &mut SessionType;
}