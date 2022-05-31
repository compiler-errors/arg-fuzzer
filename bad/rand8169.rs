
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8169(_: S1, _: S7) {}
        
        fn test8169() { foo8169(S2, S3, S0, S1, S4); }
    