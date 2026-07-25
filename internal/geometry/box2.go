package geometry

type Box2i struct {
	pos  Vector2i
	size Vector2i
}

func NewBox2i(pos, size Vector2i) Box2i {
	return Box2i{
		pos:  pos,
		size: size,
	}
}

func (b *Box2i) Get() (int32, int32, int32, int32) {
	return b.pos.x, b.pos.y, b.size.x, b.size.y
}

func (b *Box2i) GetPos() Vector2i {
	return b.pos
}

func (b *Box2i) GetSize() Vector2i {
	return b.size
}
