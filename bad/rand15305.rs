
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15305(_: S6, _: S7) {}
        
        fn test15305() { foo15305(S4, S8, S3, S2, S6, S5); }
    