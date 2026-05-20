//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1861;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta527<F: Float>(t2109: F, t26012: F, t6509: F, t7974: F, t7255: F, t7445: F, t26024: F, t1860: F, t2110: F, t22549: F, t24514: F, t24517: F, t26009: F, t26016: F, t26028: F, t26070: F, t26073: F, t26076: F, t6486: F, t7256: F, t7259: F, t7428: F, t7978: F, t33: F, t7973: F, t2240: F, t12571: F, t7245: F, t1419: F, t55: F, t22510: F, t24498: F, t3961: F, t3966: F, t607: F, t7251: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27298, t27303, t27308, t27311, t27326) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1861::<F>(t2109, t26012, t6509, t7974, t7255, t7445, t26024, t1860, t2110, t22549, t24514, t24517, t26009, t26016, t26028, t26070, t26073, t26076, t6486, t7256, t7259, t7428, t7978);
        let (t27331, t27332, t27341, t27356, t27363) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1862::<F>(t33, t7973, t2240, t12571, t7245, t1419, t55, t22510, t24498, t3961, t3966, t607, t7251);
    (t27298, t27303, t27308, t27311, t27326, t27331, t27332, t27341, t27356, t27363)
}
