
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14493(_: S6, _: S4, _: S4) {}
        
        fn test14493() { foo14493(S5, S6, S3, S3, S2); }
    