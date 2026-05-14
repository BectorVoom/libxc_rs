//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 938/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk938<F: Float>(t23164: F, t25316: F, t1519: F, t234: F, t776: F, t6637: F, t6552: F, t1894: F, t4265: F, t214: F, t1880: F, t1909: F, t226: F, t23187: F, t25277: F, t25281: F, t25285: F, t25289: F, t25293: F, t25295: F, t25297: F, t25301: F, t25304: F, t25308: F, t25310: F, t25314: F, t4162: F, t4166: F, t4281: F, t6658: F, t7535: F, t808: F, t812: F) -> (F,) {
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25320 = t25319 * t776;
    let t25321 = t6637 * t25320;
    let t25322 = t6552 * t25321;
    let t25324 = t1894 * t4265;
    let t25325 = t214 * t25324;
    let t25326 = t1880 * t25325;
    let t25328 = 0.19190897446562641759e-1 * t25277 - t4166 * t6658 + 0.41123351671205660912e-2 * t23187 + 2.0 * t4281 * t25281 - 0.82246703342411321825e-2 * t25285 + 0.16449340668482264365e-1 * t25289 + t808 * t7535 + t4162 * t1909 - 0.19190897446562641759e-1 * t25293 + t226 * t25295 - t812 * t25297 + 0.16449340668482264365e-1 * t25301 + 0.16449340668482264365e-1 * t25304 - 0.16449340668482264365e-1 * t25308 + 0.38381794893125283518e-1 * t25310 - 0.16449340668482264365e-1 * t25314 + 0.82246703342411321825e-2 * t25317 - 0.16449340668482264365e-1 * t25322 + 0.82246703342411321825e-2 * t25326;
    (t25328,)
}
