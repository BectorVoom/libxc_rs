//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1299/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1299<F: Float>(t30465: F, t580: F, t1858: F, t8240: F, t30500: F, t576: F, t2186: F, t671: F, t110631: F, t110684: F, t111594: F, t12524: F, t1458: F, t16524: F, t19534: F, t20176: F, t28893: F, t29993: F, t29996: F, t30180: F, t30253: F, t30258: F, t30424: F, t30492: F, t30495: F, t33185: F, t3938: F, t3941: F, t5371: F, t5376: F, t5456: F, t5493: F, t577: F, t75795: F, t8143: F, t8161: F, t8251: F) -> (F, F, F, F) {
    let t111601 = t30465 * t580;
    let t111602 = t8240 * t1858;
    let t111604 = t576 * t30500;
    let t111636 = t2186 * t671;
    let t111641 = F::new(54.0) * t33185 * t30253 + F::new(54.0) * t29996 * t20176 + F::new(54.0) * t12524 * t30492 + F::new(54.0) * t16524 * t30253 + F::new(54.0) * t75795 * t8251 + F::new(0.135e2) * t8161 * t19534 + F::new(54.0) * t110631 * t5376 + F::new(27.0) * t3941 * t30424 * t671 + F::new(54.0) * t16524 * t30258 + F::new(0.135e2) * t29993 * t5493 + F::new(0.135e2) * t3938 * t30424 + F::new(27.0) * t28893 * t8143 + F::new(0.45e1) * t111594 * t577 + F::new(27.0) * t110684 * t1458 + F::new(27.0) * t12524 * t30495 + F::new(27.0) * t111636 * t5456 + F::new(27.0) * t5371 * t30180;
    (t111601, t111602, t111604, t111641)
}
