//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1470;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta388<F: Float>(t16814: F, t17048: F, t858: F, t225: F, t5559: F, t5657: F, t865: F, t2718: F, t17022: F, t218: F, t5636: F, t10110: F, t1527: F, t4300: F, t259: F, t2597: F, t2713: F, t4147: F, t4268: F, t4273: F, t4301: F, t5637: F, t5658: F, t855: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17049, t17050, t17052, t17056, t17057, t17060, t17063, t17064) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1470::<F>(t16814, t17048, t858, t225, t5559, t5657, t865, t2718, t17022, t218, t5636, t10110);
        let (t17069, t17070, t17079) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1471::<F>(t1527, t4300, t2718, t17050, t17052, t17057, t17060, t17064, t259, t2597, t2713, t4147, t4268, t4273, t4301, t5637, t5658, t855, t866);
    (t17049, t17050, t17052, t17056, t17057, t17060, t17063, t17064, t17069, t17070, t17079)
}
