
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10502(_: S6, _: S5, _: S7) {}
        
        fn test10502() { foo10502(S2, S5, S1, S6, S4); }
    