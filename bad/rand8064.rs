
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8064(_: S4, _: S8) {}
        
        fn test8064() { foo8064(S7, S3, S4, S1, S0); }
    