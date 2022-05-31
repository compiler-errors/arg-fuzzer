
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo68(_: S7, _: S3, _: S5, _: S2, _: S8, _: S4, _: S6) {}
        
        fn test68() { foo68(S2, S4, S2, S4, S3); }
    