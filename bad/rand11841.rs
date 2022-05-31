
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11841(_: S6, _: S7, _: S2, _: S5) {}
        
        fn test11841() { foo11841(S3, S3, S6, S0, S5, S2); }
    