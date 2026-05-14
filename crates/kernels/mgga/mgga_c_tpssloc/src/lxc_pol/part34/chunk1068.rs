//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1068/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1068<F: Float>(t102587: F, t107281: F, t107303: F, t107320: F, t107326: F, t1825: F, t20473: F, t27074: F, t5334: F, t5344: F, t84480: F, t84481: F, t90807: F, t90837: F, t90868: F, t90900: F, t96937: F, t96945: F, t96989: F, t97026: F, t97043: F, t97049: F) -> (F,) {
    let t107908 = -0.23029076935875170111e0 * t96937 - 0.76763589786250567036e0 * t90807 + 0.11514538467937585055e0 * t96945 - 0.49348022005446793095e-1 * t107281 - 0.31253747270116302294e0 * t90837 + 6.0 * t5334 * t27074 * t20473 + 0.38381794893125283518e0 * t90868 - 3.0 * t5344 * t102587 * t1825 + 0.9869604401089358619e-1 * t107303 + 0.24674011002723396548e-1 * t96989 + 0.15626873635058151147e0 * t90900 - t84480 - t84481 - 0.16449340668482264365e-1 * t107320 + 0.49348022005446793095e-1 * t97026 - 0.9869604401089358619e-1 * t97043 - 0.49348022005446793095e-1 * t97049 - 0.49348022005446793095e-1 * t107326;
    (t107908,)
}
