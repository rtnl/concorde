package geometry

type Vector2i struct {
	x int32
	y int32
}

func NewVector2i(x, y int32) Vector2i {
	return Vector2i{
		x: x,
		y: y,
	}
}

func (v *Vector2i) Get() (int32, int32) {
	return v.x, v.y
}

func (v *Vector2i) GetX() int32 {
	return v.x
}

func (v *Vector2i) GetY() int32 {
	return v.y
}

func (v *Vector2i) Add(n int32) Vector2i {
	return NewVector2i(v.x+n, v.y+n)
}
