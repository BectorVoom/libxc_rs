//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1676;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta491<F: Float>(t26361: F, t225: F, t7919: F, t2085: F, t5210: F, t1824: F, t5250: F, t1352: F, t26393: F, t1825: F, t24116: F, t26406: F, t1336: F, t22707: F, t24099: F, t26379: F, t26381: F, t26386: F, t26390: F, t26398: F, t26412: F, t26416: F, t26419: F, t26424: F, t26427: F, t3777: F, t5234: F, t5334: F, t5344: F, t7209: F, t7932: F) -> (F, F, F, F, F, F, F, F) {
        let (t27067, t27068, t27070, t27074, t27075, t27078, t27082, t27086, t27088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1676::<F>(t26361, t225, t7919, t2085, t5210, t1824, t5250, t1352, t26393, t1825, t24116, t26406);
        let t27095 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1677::<F>(t1336, t22707, t24099, t26379, t26381, t26386, t26390, t26398, t26412, t26416, t26419, t26424, t26427, t27075, t27078, t27082, t27086, t27088, t3777, t5234, t5334, t5344, t7209, t7932);
    (t27067, t27068, t27070, t27074, t27075, t27078, t27086, t27095)
}
