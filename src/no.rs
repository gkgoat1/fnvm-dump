pub unsafe fn yes_i_am_special<'a,T>(mut a: &'a T) -> &'a mut T{
    return *std::mem::transmute::<&mut &T,&mut &mut T>(&mut a);
}