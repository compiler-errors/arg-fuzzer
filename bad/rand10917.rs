
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10917(_: S2, _: S5, _: S7) {}
        
        fn test10917() { foo10917(S3, S2, S4, S8); }
    