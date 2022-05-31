
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10741(_: S6, _: S7) {}
        
        fn test10741() { foo10741(S1, S2, S3, S4, S5, S6, S8); }
    