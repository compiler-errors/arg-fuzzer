
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo17087(_: S7, _: S2) {}
        
        fn test17087() { foo17087(S1, S4, S4); }
    