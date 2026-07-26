package structure

import "github.com/samber/mo"

type Node[T any] struct {
	value    T
	children []*Node[T]
}

func NewNode[T any](value T) (node *Node[T]) {
	node = new(Node[T])

	node.value = value
	node.children = make([]*Node[T], 0)

	return
}

func (n *Node[T]) GetValue() T {
	return n.value
}

func (n *Node[T]) SetValue(value T) {
	n.value = value
}

func (n *Node[T]) GetChild(index int) mo.Option[*Node[T]] {
	if index < 0 || index >= len(n.children) {
		return mo.None[*Node[T]]()
	}

	return mo.Some(n.children[index])
}

func (n *Node[T]) ApplyChildren(fn func(*Node[T])) {
	for _, child := range n.children {
		fn(child)
	}
}
