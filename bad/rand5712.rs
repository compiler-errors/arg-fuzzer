
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5712(_: S1, _: S4, _: S5, _: S6, _: S7) {}
        
        fn test5712() { foo5712(S5, S1, S3, S0, S7, S5, S4); }
    