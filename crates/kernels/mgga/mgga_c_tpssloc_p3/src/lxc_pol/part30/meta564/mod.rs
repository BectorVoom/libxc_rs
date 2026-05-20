//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1929;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta564<F: Float>(t28378: F, t28405: F, t235: F, t5612: F, t6657: F, t5617: F, t23008: F, t5585: F, t16758: F, t232: F, t6646: F, t1888: F, t17030: F, t16815: F, t2632: F, t22996: F, t1909: F, t226: F, t23174: F, t25310: F, t26613: F, t26667: F, t26673: F, t5575: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28406, t28407, t28409, t28411, t28413, t28418, t28419, t28420) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1929::<F>(t28378, t28405, t235, t5612, t6657, t5617, t23008, t5585, t16758, t232, t6646, t1888);
        let (t28422, t28423, t28426, t28427, t28430) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1930::<F>(t17030, t232, t6646, t1888, t16815, t2632, t22996, t1909, t226, t23174, t25310, t26613, t26667, t26673, t28407, t28409, t28411, t28413, t28420, t5575, t812);
    (t28406, t28407, t28409, t28411, t28413, t28418, t28419, t28422, t28423, t28426, t28427, t28430)
}
