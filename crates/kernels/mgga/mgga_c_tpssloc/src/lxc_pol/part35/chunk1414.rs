//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1414/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1414<F: Float>(t1992: F, t550: F, t6976: F, t74941: F, t22897: F, t3792: F, t1336: F, t1825: F, t19815: F, t20473: F, t20490: F, t20495: F, t20554: F, t22709: F, t26403: F, t26458: F, t28174: F, t5234: F, t5334: F, t6378: F, t6415: F, t6987: F, t7745: F, t7747: F, t81243: F, t90807: F, t90837: F, t90868: F, t90900: F, t96937: F, t96945: F, t96989: F, t97193: F) -> F {
    let t107281 = t1992 * t6976 * t74941 * t550;
    let t107303 = t1992 * t22897 * t74941 * t3792;
    let t107314 = -F::new(0.11514538467937585055e0) * t96937 - F::new(0.38381794893125283518e0) * t90807 + F::new(0.57572692339687925277e-1) * t96945 - F::new(0.24674011002723396548e-1) * t107281 + F::new(6.0) * t5334 * t26403 * t20473 - F::new(0.15626873635058151147e0) * t90837 - F::new(6.0) * t1336 * t81243 * t20490 + F::new(6.0) * t1336 * t22709 * t20495 - F::new(3.0) * t1336 * t97193 * t1825 + F::new(0.19190897446562641759e0) * t90868 - t1336 * t6987 * t20554 - F::new(3.0) * t19815 * t7745 + F::new(0.49348022005446793095e-1) * t107303 + F::new(0.12337005501361698274e-1) * t96989 + F::new(3.0) * t6378 * t7747 - F::new(3.0) * t1336 * t26458 * t6415 + F::new(0.78134368175290755733e-1) * t90900 - F::new(3.0) * t5234 * t28174;
    t107314
}
