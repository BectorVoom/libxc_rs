//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta794<F: Float>(t40767: F, t16689: F, t2655: F, t46302: F, t16701: F, t2427: F, t13133: F, t4101: F, t10126: F, t16949: F, t2522: F, t2523: F, t39529: F, t40764: F, t40766: F, t40779: F, t40784: F, t4314: F, t5544: F) -> (F, F, F, F, F, F) {
        let (t58040, t58042, t58046, t58048, t58053, t58054) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2755::<F>(t40767, t16689, t2655, t46302, t16701, t2427, t13133, t4101, t10126, t16949, t2522, t2523, t39529, t40764, t40766, t40779, t40784, t4314, t5544);
    (t58040, t58042, t58046, t58048, t58053, t58054)
}
