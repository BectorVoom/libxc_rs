//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2348/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2348<F: Float>(t254: F, t563: F, t1215: F, t1409: F, t460: F, t4928: F, t492: F, t64: F, t9365: F, t1444: F, t659: F, t1449: F, t662: F) -> (F, F, F, F, F, F, F) {
    let t26224 = t563 * t254;
    let t27524 = t1409 * t1215;
    let t27654 = t4928 * t460;
    let t27784 = t492 * t254;
    let t29903 = t64 * t9365;
    let t30171 = t1444 * t659;
    let t30307 = t1449 * t662;
    (t26224, t27524, t27654, t27784, t29903, t30171, t30307)
}
