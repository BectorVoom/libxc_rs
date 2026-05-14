//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1226/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1226<F: Float>(t10195: F, t13784: F, t2986: F, t1887: F, t2262: F, t337: F, t10186: F, t10191: F, t13783: F, t984: F, t10237: F, t10277: F, t343: F, t9288: F, t3014: F, t4509: F) -> (F, F, F, F, F, F) {
    let t42827 = t2986 * t13784 * t10195;
    let t42830 = t2262 * t337 * t1887;
    let t42833 = t10186 * t10191;
    let t42837 = t13783 * t984;
    let t42839 = t2986 * t42837 * t10237;
    let t42841 = t343 * t10277;
    let t42842 = t42841 * t9288;
    let t42846 = t4509 * t3014;
    (t42827, t42830, t42833, t42839, t42842, t42846)
}
