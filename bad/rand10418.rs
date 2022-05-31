
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10418(_: S1, _: S6) {}
        
        fn test10418() { foo10418(S7, S5, S7, S1, S7, S7); }
    