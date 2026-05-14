//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1288/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1288<F: Float>(t6224: F, t8054: F, t103959: F, t109385: F, t11881: F, t11883: F, t11888: F, t11889: F, t15027: F, t2121: F, t2147: F, t21769: F, t21776: F, t22327: F, t27406: F, t29705: F, t29712: F, t29723: F, t29790: F, t3624: F, t3625: F, t462: F, t5064: F, t6140: F, t6168: F, t7283: F, t7362: F, t7363: F, t8077: F, t8085: F, t95726: F) -> (F, F) {
    let t109418 = t8054 * t6224;
    let t109432 = 3.0 * t6168 * t8085 + 0.14621636149762012769e-1 * t103959 - 0.27415567780803773942e-2 * t7283 * t7362 * t7363 * t21776 - 0.54831135561607547884e-2 * t95726 + 6.0 * t15027 * t29723 + 3.0 * t5064 * t29712 - 0.16449340668482264365e-1 * t7283 * t7362 * t7363 * t21769 - 0.24674011002723396548e-1 * t7283 * t6140 * t8077 + 0.82246703342411321825e-2 * t2121 * t462 * t2147 * t22327 - 3.0 * t3624 * t109418 * t3625 - 6.0 * t11888 * t109385 * t11889 + 0.65797362673929057459e-1 * t27406 * t29705 + 0.13159472534785811492e0 * t27406 * t29790 + 6.0 * t11881 * t109385 * t11883;
    (t109418, t109432)
}
