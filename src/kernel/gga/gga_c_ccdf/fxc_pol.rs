//! GGA_C_CCDF fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT6, M_PI};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_ccdf_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = param_c2 * t3 + 1.0;
        let t6 = 1.0 / t5;
        let t7 = param_c1 * t6;
        let t8 = M_CBRT2;
        let t9 = M_CBRT6;
        let t10 = t9 * t9;
        let t11 = t8 * t10;
        let t12 = M_PI * M_PI;
        let t13 = pow_1_3(t12);
        let t14 = 1.0 / t13;
        let t16 = sigma0 + 2.0 * sigma1 + sigma2;
        let t17 = f64::sqrt(t16);
        let t18 = t14 * t17;
        let t20 = 1.0 / t2 / t1;
        let t26 = f64::exp(-param_c4 * (t11 * t18 * t20 / 12.0 - param_c5));
        let t27 = 1.0 + t26;
        let t30 = 1.0 - param_c3 / t27;
        let tzk0 = t7 * t30;
        zk[ip] += tzk0;
        let t31 = t3 * param_c1;
        let t32 = t5 * t5;
        let t33 = 1.0 / t32;
        let t39 = t6 * param_c3;
        let t40 = t27 * t27;
        let t41 = 1.0 / t40;
        let t42 = t39 * t41;
        let t43 = t20 * param_c1 * t42;
        let t45 = param_c4 * t8 * t10;
        let tvrho0 = tzk0 + t31 * t33 * t30 * param_c2 / 3.0 + t43 * t45 * t18 * t26 / 9.0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t50 = t31 * t42;
        let t51 = 1.0 / t17;
        let t54 = t45 * t14 * t51 * t26;
        let t55 = t50 * t54;
        let tvsigma0 = -t55 / 24.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -t55 / 12.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t58 = param_c1 * t33;
        let t59 = t30 * param_c2;
        let t63 = param_c3 * t41;
        let t64 = t63 * param_c4;
        let t65 = t7 * t64;
        let t66 = t11 * t14;
        let t67 = t1 * t1;
        let t69 = 1.0 / t2 / t67;
        let t75 = t2 * t2;
        let t78 = 1.0 / t75 / t1 * param_c1;
        let t80 = 1.0 / t32 / t5;
        let t82 = param_c2 * param_c2;
        let t87 = 1.0 / t75 / t67;
        let t88 = t87 * param_c1;
        let t90 = t88 * t33 * t64;
        let t91 = t17 * t26;
        let t92 = t91 * param_c2;
        let t96 = t67 * t1;
        let t98 = 1.0 / t75 / t96;
        let t99 = t98 * param_c1;
        let t101 = 1.0 / t40 / t27;
        let t102 = t39 * t101;
        let t103 = t99 * t102;
        let t104 = param_c4 * param_c4;
        let t105 = t8 * t8;
        let t106 = t104 * t105;
        let t107 = t106 * t9;
        let t108 = t13 * t13;
        let t109 = 1.0 / t108;
        let t110 = t109 * t16;
        let t111 = t26 * t26;
        let t116 = t99 * t42;
        let tv2rho20 = 2.0 / 9.0 * t58 * t59 * t20 - t65 * t66 * t17 * t69 * t26 / 27.0 + 2.0 / 9.0 * t78 * t80 * t30 * t82 + 2.0 / 27.0 * t90 * t66 * t92 - 4.0 / 27.0 * t103 * t107 * t110 * t111 + 2.0 / 27.0 * t116 * t107 * t110 * t26;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t121 = t43 * t54;
        let t124 = t78 * t33 * t64;
        let t125 = t51 * t26;
        let t126 = t125 * param_c2;
        let t127 = t66 * t126;
        let t128 = t124 * t127;
        let t130 = t88 * t102;
        let t131 = t9 * t109;
        let t133 = t106 * t131 * t111;
        let t134 = t130 * t133;
        let t136 = t88 * t42;
        let t138 = t106 * t131 * t26;
        let t139 = t136 * t138;
        let tv2rhosigma0 = t121 / 72.0 - t128 / 72.0 + t134 / 18.0 - t139 / 36.0;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = t121 / 36.0 - t128 / 36.0 + t134 / 9.0 - t139 / 18.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = tv2rhosigma2;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = tv2rhosigma1;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t145 = t78 * t102;
        let t146 = 1.0 / t16;
        let t147 = t109 * t146;
        let t149 = t107 * t147 * t111;
        let t150 = t145 * t149;
        let t152 = t17 * t16;
        let t153 = 1.0 / t152;
        let t156 = t45 * t14 * t153 * t26;
        let t157 = t50 * t156;
        let t159 = t78 * t42;
        let t161 = t107 * t147 * t26;
        let t162 = t159 * t161;
        let tv2sigma20 = -t150 / 48.0 + t157 / 48.0 + t162 / 96.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = -t150 / 24.0 + t157 / 24.0 + t162 / 48.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = tv2sigma20;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = -t150 / 12.0 + t157 / 12.0 + t162 / 24.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = tv2sigma21;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = tv2sigma22;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
