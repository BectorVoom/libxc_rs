//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1516;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1517;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1518;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta368<F: Float>(t13602: F, t12606: F, t883: F, t882: F, t123: F, t10556: F, t10558: F, t10560: F, t10562: F, t10577: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13600: F, t13601: F, t901: F, t2815: F, t4370: F, t896: F, t2807: F, t4378: F, t2798: F, t4362: F, t10595: F, t1547: F, t2799: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13603, t13611) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1516::<F>(t13602, t12606, t883);
        let t13613 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1517::<F>(t13611, t882, t123);
        let t13615 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1518::<F>(t10556, t10558, t10560, t10562, t10577, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13600, t13601, t13603, t13613);
        let (t13616, t13624, t13626, t13630, t13632, t13635) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1519::<F>(t13615, t901, t2815, t4370, t896, t2807, t4378, t2798, t4362, t10595, t1547, t2799);
    (t13611, t13613, t13615, t13616, t13624, t13626, t13630, t13632, t13635)
}
