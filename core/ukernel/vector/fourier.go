package vector

import . "ukernel/number"

type Fourier struct {
	N, Log2N Bin32u
	Vector   []Complex
}

func (F Fourier) Unities(N Bin32u) Fourier {
	unities := make([]Complex, N)
	for i := Zero32u; i <= N>>One32u; i++ {
		unities[i] = unities[i].Arg(i, N)
	}
	return Fourier{
		N:      N,
		Log2N:  N.LogBin(),
		Vector: unities,
	}
}

func (F Fourier) butterfly() Fourier {
	Y := Fourier{
		N:      F.N,
		Log2N:  F.Log2N,
		Vector: make([]Complex, F.N),
	}
	for i := Zero32u; i < F.N; i++ { // bit reversal of the given array
		Y.Vector[i] = F.Vector[i.BitReverse()]
	}
	return Y
}

func (F Fourier) Dft() Fourier {
	Y := F.butterfly()
	J := Complex{0, 1} // j is iota
	for s := One32u; s <= F.Log2N; s++ {
		m := One32u << s  // 2 power s
		m2 := m >> One32u // m2 = m/2 -1
		w := J
		wm := J.Arg(m2, F.N) // principle root of nth complex - root of unity.
		for j := Zero32u; j < m2; j++ {
			for k := j; k < F.N; k += m {
				t := w.Mul(F.Vector[k+m2]) // t = twiddle factor
				u := Y.Vector[k]
				Y.Vector[k] = u.Add(t)    // calculating y[k]
				Y.Vector[k+m2] = u.Sub(t) // similar calculating y[k+n/2]
			}
			w.Mul(wm)
		}
	}
	return Y
}

/*
func Idft(polynom, Domain []Complex) Number {
	N := uint32(len(polynom))
	idft := Number{
		Vector: make([]int8, N),
		Len:    N,
	}
	for i := Zero32; i <= N - 1; i++ {
		idft.Vector[i] = sumReal(mulComplex(polynom[i], Domain[i]))
	}
}
*/

func (F Fourier) DftWithMemory(Domain Fourier) Fourier {
	Y := F.butterfly()
	for s := One32u; s <= F.Log2N; s++ {
		m := One32u << s  // 2 power s
		m2 := m >> One32u // m2 = m/2 -1
		for j := Zero32u; j < m2; j++ {
			for k := j; k < F.N; k += m {
				t := Domain.Vector[m2].Mul(F.Vector[k+m2]) // t = twiddle factor
				u := Y.Vector[k]
				Y.Vector[k] = u.Add(t)    // calculating y[k]
				Y.Vector[k+m2] = u.Sub(t) // similar calculating y[k+n/2]
			}
		}
	}
	return Y
}

/*
func IdftWithMemory(polynom, Domain []Complex) Number {
	N := uint32(len(polynom))
	idft := Number{
		Vector: make([]int8, N),
		Len:    N,
	}
	for i := Zero32; i <= N - 1; i++ {
		idft.Vector[i] = sumReal(mulComplex(polynom[i], Domain[i]))
	}
}
*/
