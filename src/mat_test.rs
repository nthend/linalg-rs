#[allow(unused_imports)]
use mat::*;
#[allow(unused_imports)]
use vec::*;

macro_rules! mat_new_test {
	($V:ident, $N:expr, $M:expr) => (
		$V::<i32>::new();
	)
}

#[test]
fn new() {
	mat_new_test!(Mat2x2, 2, 2);
	mat_new_test!(Mat2x3, 2, 3);
	mat_new_test!(Mat2x4, 2, 4);
	mat_new_test!(Mat3x2, 3, 2);
	mat_new_test!(Mat3x3, 3, 3);
	mat_new_test!(Mat3x4, 3, 4);
	mat_new_test!(Mat4x2, 4, 2);
	mat_new_test!(Mat4x3, 4, 3);
	mat_new_test!(Mat4x4, 4, 4);
}

macro_rules! mat_content_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::new_map(|i, j| i + j);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m.d[i + $N*j], i + j);
			}
		}

		let z = $V::new_scal(0);
		for i in 0..($N*$M) {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..($N*$M) {
			m.d[i] = i + 2;
		}
		for i in 0..($N*$M) {
			assert_eq!(m.d[i], i + 2);
		}
	)
}

#[test]
fn content() {
	mat_content_test!(Mat2x2, 2, 2);
	mat_content_test!(Mat2x3, 2, 3);
	mat_content_test!(Mat2x4, 2, 4);
	mat_content_test!(Mat3x2, 3, 2);
	mat_content_test!(Mat3x3, 3, 3);
	mat_content_test!(Mat3x4, 3, 4);
	mat_content_test!(Mat4x2, 4, 2);
	mat_content_test!(Mat4x3, 4, 3);
	mat_content_test!(Mat4x4, 4, 4);
}

macro_rules! mat_data_test {
	($V:ident, $N:expr, $M:expr) => (
		let v = $V::new_map(|i, j| i + j + 1);

		let a = &v.d;
		let b = v.data(); 
		for i in 0..($N*$M) {
			assert_eq!(a[i], b[i]);
		}
	)
}

#[test]
fn data() {
	mat_data_test!(Mat2x2, 2, 2);
	mat_data_test!(Mat2x3, 2, 3);
	mat_data_test!(Mat2x4, 2, 4);
	mat_data_test!(Mat3x2, 3, 2);
	mat_data_test!(Mat3x3, 3, 3);
	mat_data_test!(Mat3x4, 3, 4);
	mat_data_test!(Mat4x2, 4, 2);
	mat_data_test!(Mat4x3, 4, 3);
	mat_data_test!(Mat4x4, 4, 4);
}

macro_rules! mat_eq_test {
	($V:ident, $N:expr, $M:expr) => (
		let va = $V::new_map(|i, j| i + j);
		let vb = $V::new_map(|i, j| i + j);
		assert_eq!(va, vb);
	)
}

#[test]
fn eq() {
	mat_eq_test!(Mat2x2, 2, 2);
	mat_eq_test!(Mat2x3, 2, 3);
	mat_eq_test!(Mat2x4, 2, 4);
	mat_eq_test!(Mat3x2, 3, 2);
	mat_eq_test!(Mat3x3, 3, 3);
	mat_eq_test!(Mat3x4, 3, 4);
	mat_eq_test!(Mat4x2, 4, 2);
	mat_eq_test!(Mat4x3, 4, 3);
	mat_eq_test!(Mat4x4, 4, 4);
}

macro_rules! mat_copy_test {
	($V:ident, $N:expr, $M:expr) => (
		let m = $V::new_map(|i, j| i + j);
		let cm = m;
		assert_eq!(m, cm);
	)
}

#[test]
fn copy() {
	mat_copy_test!(Mat2x2, 2, 2);
	mat_copy_test!(Mat2x3, 2, 3);
	mat_copy_test!(Mat2x4, 2, 4);
	mat_copy_test!(Mat3x2, 3, 2);
	mat_copy_test!(Mat3x3, 3, 3);
	mat_copy_test!(Mat3x4, 3, 4);
	mat_copy_test!(Mat4x2, 4, 2);
	mat_copy_test!(Mat4x3, 4, 3);
	mat_copy_test!(Mat4x4, 4, 4);
}
/*
macro_rules! mat_index_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::<usize> { d: mat_arr![i, j; i + j; $N, $M] };
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i + j);
			}
		}
		for j in 0..$M {
			for i in 0..$N {
				m[(i, j)] = i*j;
			}
		}
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn index() {
	mat_index_test!(Mat2x2, 2, 2);
	mat_index_test!(Mat2x3, 2, 3);
	mat_index_test!(Mat2x4, 2, 4);
	mat_index_test!(Mat3x2, 3, 2);
	mat_index_test!(Mat3x3, 3, 3);
	mat_index_test!(Mat3x4, 3, 4);
	mat_index_test!(Mat4x2, 4, 2);
	mat_index_test!(Mat4x3, 4, 3);
	mat_index_test!(Mat4x4, 4, 4);
}

macro_rules! mat_zero_test {
	($V:ident, $N:expr, $M:expr) => (
		let z = $V::<i32>::zero();
		for i in 0..($N*$M) {
			assert_eq!(z.d[i], 0);
		}
		assert!(z.is_zero());
		
		let nz = $V::<i32> { d: [1; ($N*$M)] };
		assert!(!nz.is_zero());
	)
}

#[test]
fn zero() {
	mat_zero_test!(Mat2x2, 2, 2);
	mat_zero_test!(Mat2x3, 2, 3);
	mat_zero_test!(Mat2x4, 2, 4);
	mat_zero_test!(Mat3x2, 3, 2);
	mat_zero_test!(Mat3x3, 3, 3);
	mat_zero_test!(Mat3x4, 3, 4);
	mat_zero_test!(Mat4x2, 4, 2);
	mat_zero_test!(Mat4x3, 4, 3);
	mat_zero_test!(Mat4x4, 4, 4);
}

macro_rules! mat_from_test {
	($V:ident, $N:expr, $M:expr) => (
		let mf: $V<usize> = $V::from(mat_arr![i, j; i + j; $N, $M]);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(mf[(i, j)], i + j);
			}
		}

		let mi: $V<usize> = mat_arr![i, j; i*j; $N, $M].into();
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(mi[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn from() {
	mat_from_test!(Mat2x2, 2, 2);
	mat_from_test!(Mat2x3, 2, 3);
	mat_from_test!(Mat2x4, 2, 4);
	mat_from_test!(Mat3x2, 3, 2);
	mat_from_test!(Mat3x3, 3, 3);
	mat_from_test!(Mat3x4, 3, 4);
	mat_from_test!(Mat4x2, 4, 2);
	mat_from_test!(Mat4x3, 4, 3);
	mat_from_test!(Mat4x4, 4, 4);
}

macro_rules! mat_neg_test {
	($V:ident, $N:expr, $M:expr) => (
		let m: $V<i32> = mat_arr![i, j; (i + j + 1) as i32; $N, $M].into();
		let nm = -m;
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(-m[(i, j)], nm[(i, j)]);
			}
		}
	)
}

#[test]
fn neg() {
	mat_neg_test!(Mat2x2, 2, 2);
	mat_neg_test!(Mat2x3, 2, 3);
	mat_neg_test!(Mat2x4, 2, 4);
	mat_neg_test!(Mat3x2, 3, 2);
	mat_neg_test!(Mat3x3, 3, 3);
	mat_neg_test!(Mat3x4, 3, 4);
	mat_neg_test!(Mat4x2, 4, 2);
	mat_neg_test!(Mat4x3, 4, 3);
	mat_neg_test!(Mat4x4, 4, 4);
}

macro_rules! mat_op_mat_test {
	($V:ident, $N:expr, $M:expr, $op:ident) => (
		let va = mat_map![i, j; (2*(i + j) + 2) as i32; $V, $N, $M];
		let vb = mat_map![i, j; (i + j + 1) as i32; $V, $N, $M];
		let vc = $op!(va, vb);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(vc[(i, j)], $op!(va[(i, j)], vb[(i, j)]));
			}
		}
	)
}

#[test]
fn mat_add() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_add);
	mat_op_mat_test!(Mat2x3, 2, 3, op_add);
	mat_op_mat_test!(Mat2x4, 2, 4, op_add);
	mat_op_mat_test!(Mat3x2, 3, 2, op_add);
	mat_op_mat_test!(Mat3x3, 3, 3, op_add);
	mat_op_mat_test!(Mat3x4, 3, 4, op_add);
	mat_op_mat_test!(Mat4x2, 4, 2, op_add);
	mat_op_mat_test!(Mat4x3, 4, 3, op_add);
	mat_op_mat_test!(Mat4x4, 4, 4, op_add);
}

#[test]
fn mat_sub() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_sub);
	mat_op_mat_test!(Mat2x3, 2, 3, op_sub);
	mat_op_mat_test!(Mat2x4, 2, 4, op_sub);
	mat_op_mat_test!(Mat3x2, 3, 2, op_sub);
	mat_op_mat_test!(Mat3x3, 3, 3, op_sub);
	mat_op_mat_test!(Mat3x4, 3, 4, op_sub);
	mat_op_mat_test!(Mat4x2, 4, 2, op_sub);
	mat_op_mat_test!(Mat4x3, 4, 3, op_sub);
	mat_op_mat_test!(Mat4x4, 4, 4, op_sub);
}

macro_rules! mat_op_scal_test {
	($V:ident, $N:expr, $M:expr, $op:ident) => (
		let v = mat_map![i, j; (2*(i + j) + 2) as i32; $V, $N, $M];
		let a = 3;
		let va = $op!(v, a);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(va[(i, j)], $op!(v[(i, j)], a));
			}
		}
	)
}

#[test]
fn scal_mul() {
	mat_op_scal_test!(Mat2x2, 2, 2, op_mul);
	mat_op_scal_test!(Mat2x3, 2, 3, op_mul);
	mat_op_scal_test!(Mat2x4, 2, 4, op_mul);
	mat_op_scal_test!(Mat3x2, 3, 2, op_mul);
	mat_op_scal_test!(Mat3x3, 3, 3, op_mul);
	mat_op_scal_test!(Mat3x4, 3, 4, op_mul);
	mat_op_scal_test!(Mat4x2, 4, 2, op_mul);
	mat_op_scal_test!(Mat4x3, 4, 3, op_mul);
	mat_op_scal_test!(Mat4x4, 4, 4, op_mul);
}

#[test]
fn scal_div() {
	mat_op_scal_test!(Mat2x2, 2, 2, op_div);
	mat_op_scal_test!(Mat2x3, 2, 3, op_div);
	mat_op_scal_test!(Mat2x4, 2, 4, op_div);
	mat_op_scal_test!(Mat3x2, 3, 2, op_div);
	mat_op_scal_test!(Mat3x3, 3, 3, op_div);
	mat_op_scal_test!(Mat3x4, 3, 4, op_div);
	mat_op_scal_test!(Mat4x2, 4, 2, op_div);
	mat_op_scal_test!(Mat4x3, 4, 3, op_div);
	mat_op_scal_test!(Mat4x4, 4, 4, op_div);
}

#[test]
fn scal_rem() {
	mat_op_scal_test!(Mat2x2, 2, 2, op_rem);
	mat_op_scal_test!(Mat2x3, 2, 3, op_rem);
	mat_op_scal_test!(Mat2x4, 2, 4, op_rem);
	mat_op_scal_test!(Mat3x2, 3, 2, op_rem);
	mat_op_scal_test!(Mat3x3, 3, 3, op_rem);
	mat_op_scal_test!(Mat3x4, 3, 4, op_rem);
	mat_op_scal_test!(Mat4x2, 4, 2, op_rem);
	mat_op_scal_test!(Mat4x3, 4, 3, op_rem);
	mat_op_scal_test!(Mat4x4, 4, 4, op_rem);
}


macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! mat_op_mat_assign_test {
	($V:ident, $N:expr, $M:expr, $op_assign:ident, $op:ident) => (
		let va = mat_map![i, j; (2*(i + 2*j) + 2) as i32; $V, $N, $M];
		let vb = mat_map![i, j; (i + j + 1) as i32; $V, $N, $M];
		let mut vc = va;
		$op_assign!(vc, vb);
		assert_eq!(vc, $op!(va, vb));
	)
}

#[test]
fn mat_add_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_add_assign, op_add);
}

#[test]
fn mat_sub_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_sub_assign, op_sub);
}

macro_rules! mat_op_scal_assign_test {
	($V:ident, $N:expr, $M:expr, $op_assign:ident, $op:ident) => (
		let v = mat_map![i, j; (2*(i + 2*j) + 2) as i32; $V, $N, $M];
		let a = 3;
		let mut va = v;
		$op_assign!(va, a);
		assert_eq!(va, $op!(v, a));
	)
}

#[test]
fn scal_mul_assign() {
	mat_op_scal_assign_test!(Mat2x2, 2, 2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat2x3, 2, 3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat2x4, 2, 4, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat3x2, 3, 2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat3x3, 3, 3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat3x4, 3, 4, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat4x2, 4, 2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat4x3, 4, 3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat4x4, 4, 4, op_mul_assign, op_mul);
}

#[test]
fn scal_div_assign() {
	mat_op_scal_assign_test!(Mat2x2, 2, 2, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat2x3, 2, 3, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat2x4, 2, 4, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat3x2, 3, 2, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat3x3, 3, 3, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat3x4, 3, 4, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat4x2, 4, 2, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat4x3, 4, 3, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat4x4, 4, 4, op_div_assign, op_div);
}

#[test]
fn scal_rem_assign() {
	mat_op_scal_assign_test!(Mat2x2, 2, 2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat2x3, 2, 3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat2x4, 2, 4, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat3x2, 3, 2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat3x3, 3, 3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat3x4, 3, 4, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat4x2, 4, 2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat4x3, 4, 3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat4x4, 4, 4, op_rem_assign, op_rem);
}

macro_rules! mat_transpose_test {
	($Vnm:ident, $Vmn:ident, $N:expr, $M:expr) => (
		let vnm = mat_map![i, j; 2*i + 3*j; $Vnm, $N, $M];
		let vmn = mat_map![i, j; 3*i + 2*j; $Vmn, $M, $N];
		assert_eq!(vnm.transpose(), vmn);
	)
}

#[test]
fn transpose() {
	mat_transpose_test!(Mat2x2, Mat2x2, 2, 2);
	mat_transpose_test!(Mat2x3, Mat3x2, 2, 3);
	mat_transpose_test!(Mat2x4, Mat4x2, 2, 4);
	mat_transpose_test!(Mat3x2, Mat2x3, 3, 2);
	mat_transpose_test!(Mat3x3, Mat3x3, 3, 3);
	mat_transpose_test!(Mat3x4, Mat4x3, 3, 4);
	mat_transpose_test!(Mat4x2, Mat2x4, 4, 2);
	mat_transpose_test!(Mat4x3, Mat3x4, 4, 3);
	mat_transpose_test!(Mat4x4, Mat4x4, 4, 4);
}

macro_rules! mat_outer_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let vn = vec_map![i; i as i32; $Vn, $N];
		let vm = vec_map![i; 2*i as i32; $Vm, $M];
		let mat = mat_map![i, j; (2*i*j) as i32; $Vnm, $N, $M];
		let res = vm.outer(vn);
		assert_eq!(res, mat);
	)
}

#[test]
fn outer() {
	mat_outer_test!(Mat2x2, vec2, vec2, 2, 2);
	mat_outer_test!(Mat2x3, vec2, vec3, 2, 3);
	mat_outer_test!(Mat2x4, vec2, vec4, 2, 4);
	mat_outer_test!(Mat3x2, vec3, vec2, 3, 2);
	mat_outer_test!(Mat3x3, vec3, vec3, 3, 3);
	mat_outer_test!(Mat3x4, vec3, vec4, 3, 4);
	mat_outer_test!(Mat4x2, vec4, vec2, 4, 2);
	mat_outer_test!(Mat4x3, vec4, vec3, 4, 3);
	mat_outer_test!(Mat4x4, vec4, vec4, 4, 4);
}

macro_rules! mat_mul_vec_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let m: $Vnm<i32> = [1; $N*$M].into();
		let v: $Vn<i32> = [1; $N].into();
		assert_eq!(m*v, [$N; $M].into());
	)
}

#[test]
fn mul_vec() {
	mat_mul_vec_test!(Mat2x2, vec2, vec2, 2, 2);
	mat_mul_vec_test!(Mat2x3, vec2, vec3, 2, 3);
	mat_mul_vec_test!(Mat2x4, vec2, vec4, 2, 4);
	mat_mul_vec_test!(Mat3x2, vec3, vec2, 3, 2);
	mat_mul_vec_test!(Mat3x3, vec3, vec3, 3, 3);
	mat_mul_vec_test!(Mat3x4, vec3, vec4, 3, 4);
	mat_mul_vec_test!(Mat4x2, vec4, vec2, 4, 2);
	mat_mul_vec_test!(Mat4x3, vec4, vec3, 4, 3);
	mat_mul_vec_test!(Mat4x4, vec4, vec4, 4, 4);
}

macro_rules! mat_mul_vec_mat_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let m: $Vnm<i32> = [1; $N*$M].into();
		let v: $Vm<i32> = [1; $M].into();
		assert_eq!(v*m, [$M; $N].into());
	)
}

#[test]
fn mul_mat_vec() {
	mat_mul_vec_mat_test!(Mat2x2, vec2, vec2, 2, 2);
	mat_mul_vec_mat_test!(Mat2x3, vec2, vec3, 2, 3);
	mat_mul_vec_mat_test!(Mat2x4, vec2, vec4, 2, 4);
	mat_mul_vec_mat_test!(Mat3x2, vec3, vec2, 3, 2);
	mat_mul_vec_mat_test!(Mat3x3, vec3, vec3, 3, 3);
	mat_mul_vec_mat_test!(Mat3x4, vec3, vec4, 3, 4);
	mat_mul_vec_mat_test!(Mat4x2, vec4, vec2, 4, 2);
	mat_mul_vec_mat_test!(Mat4x3, vec4, vec3, 4, 3);
	mat_mul_vec_mat_test!(Mat4x4, vec4, vec4, 4, 4);
}

macro_rules! mat_mul_mat_test {
	($Vnm:ident, $Vln:ident, $Vlm:ident, $N:expr, $M:expr, $L:expr) => (
		let vnm: $Vnm<i32> = [1; $N*$M].into();
		let vln: $Vln<i32> = [1; $L*$N].into();
		assert_eq!(vnm*vln, [$N; $L*$M].into());
	)
}

#[test]
fn mul_mat() {
	mat_mul_mat_test!(Mat2x2, Mat2x2, Mat2x2, 2, 2, 2);
	mat_mul_mat_test!(Mat2x2, Mat3x2, Mat3x2, 2, 2, 3);
	mat_mul_mat_test!(Mat2x2, Mat4x2, Mat4x2, 2, 2, 4);
	mat_mul_mat_test!(Mat2x3, Mat2x2, Mat2x3, 2, 3, 2);
	mat_mul_mat_test!(Mat2x3, Mat3x2, Mat3x3, 2, 3, 3);
	mat_mul_mat_test!(Mat2x3, Mat4x2, Mat4x3, 2, 3, 4);
	mat_mul_mat_test!(Mat2x4, Mat2x2, Mat2x4, 2, 4, 2);
	mat_mul_mat_test!(Mat2x4, Mat3x2, Mat3x4, 2, 4, 3);
	mat_mul_mat_test!(Mat2x4, Mat4x2, Mat4x4, 2, 4, 4);
	mat_mul_mat_test!(Mat3x2, Mat2x3, Mat2x2, 3, 2, 2);
	mat_mul_mat_test!(Mat3x2, Mat3x3, Mat3x2, 3, 2, 3);
	mat_mul_mat_test!(Mat3x2, Mat4x3, Mat4x2, 3, 2, 4);
	mat_mul_mat_test!(Mat3x3, Mat2x3, Mat2x3, 3, 3, 2);
	mat_mul_mat_test!(Mat3x3, Mat3x3, Mat3x3, 3, 3, 3);
	mat_mul_mat_test!(Mat3x3, Mat4x3, Mat4x3, 3, 3, 4);
	mat_mul_mat_test!(Mat3x4, Mat2x3, Mat2x4, 3, 4, 2);
	mat_mul_mat_test!(Mat3x4, Mat3x3, Mat3x4, 3, 4, 3);
	mat_mul_mat_test!(Mat3x4, Mat4x3, Mat4x4, 3, 4, 4);
	mat_mul_mat_test!(Mat4x2, Mat2x4, Mat2x2, 4, 2, 2);
	mat_mul_mat_test!(Mat4x2, Mat3x4, Mat3x2, 4, 2, 3);
	mat_mul_mat_test!(Mat4x2, Mat4x4, Mat4x2, 4, 2, 4);
	mat_mul_mat_test!(Mat4x3, Mat2x4, Mat2x3, 4, 3, 2);
	mat_mul_mat_test!(Mat4x3, Mat3x4, Mat3x3, 4, 3, 3);
	mat_mul_mat_test!(Mat4x3, Mat4x4, Mat4x3, 4, 3, 4);
	mat_mul_mat_test!(Mat4x4, Mat2x4, Mat2x4, 4, 4, 2);
	mat_mul_mat_test!(Mat4x4, Mat3x4, Mat3x4, 4, 4, 3);
	mat_mul_mat_test!(Mat4x4, Mat4x4, Mat4x4, 4, 4, 4);
}

macro_rules! mat_one_test {
	($V:ident, $N:expr) => (
		let o = $V::<i32>::one();
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(o[(i, j)], if i == j { 1 } else { 0 });
			}
		}
	)
}

#[test]
fn one() {
	mat_one_test!(Mat2x2, 2);
	mat_one_test!(Mat3x3, 3);
	mat_one_test!(Mat4x4, 4);
}

#[test]
fn det() {
	let m: Mat2<i32> = [11, 12, 21, 22].into();
	assert_eq!(m.det(), 11*22 - 12*21);
	
	let m: Mat3<i32> = [11, 12, 13, 21, 22, 23, 31, 32, 33].into();
	assert_eq!(m.det(), 11*(22*33 - 23*32) + 12*(23*31 - 21*33) + 13*(21*32 - 22*31));
}

#[test]
fn inverse() {
	let m = Mat2::<f64>::from([11.0, 12.0, 21.0, 22.0]).inverse();
	let im = Mat2::<f64>::from([22.0, -12.0, -21.0, 11.0])/(11.0*22.0 - 12.0*21.0);
	let dm = m - im;
	assert!(dm[(0, 0)].abs() + dm[(0, 1)].abs() + dm[(1, 0)].abs() + dm[(1, 1)].abs() < 1e-8);
}
*/
