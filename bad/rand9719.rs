
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9719(_: S1, _: S2) {}
        
        fn test9719() { foo9719(S8, S1, S5, S6, S7, S3); }
    