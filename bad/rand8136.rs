
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8136(_: S3, _: S4, _: S3, _: S5, _: S7, _: S3) {}
        
        fn test8136() { foo8136(S0, S1, S0, S4, S7, S2, S6); }
    